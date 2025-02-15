use crate::domain::log::LogMessage;
use crate::infrastructure::kafka_log_repository::KafkaLogRepository;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;
use tracing::{error, info};

pub struct AsyncLogQueue {
    sender: mpsc::Sender<LogMessage>,
}

impl AsyncLogQueue {
    pub fn new(kafka_repo: Arc<KafkaLogRepository>) -> Self {
        let (tx, mut rx) = mpsc::channel::<LogMessage>(1000); // ✅ Queue up to 1000 logs

        // ✅ Spawn worker task to process logs in batches
        tokio::spawn(async move {
            let mut batch = Vec::new();
            loop {
                while let Ok(log) = rx.try_recv() {
                    batch.push(log);
                    if batch.len() >= 10 {
                        // ✅ Process in batches of 10
                        break;
                    }
                }

                if !batch.is_empty() {
                    for log in batch.drain(..) {
                        let kafka_repo_clone = Arc::clone(&kafka_repo);
                        tokio::spawn(async move {
                            AsyncLogQueue::send_with_retries(kafka_repo_clone, log).await;
                        });
                    }
                }

                sleep(Duration::from_millis(500)).await; // ✅ Small delay to reduce CPU usage
            }
        });

        Self { sender: tx }
    }

    pub async fn log(&self, log: LogMessage) {
        if let Err(e) = self.sender.send(log).await {
            error!("❌ Failed to queue log: {:?}", e);
        }
    }

    async fn send_with_retries(kafka_repo: Arc<KafkaLogRepository>, log: LogMessage) {
        let retry_strategy = ExponentialBackoff::from_millis(500).map(jitter).take(5); // ✅ Retry up to 5 times

        let result = Retry::spawn(retry_strategy, || async {
            kafka_repo.send_to_kafka(&log).await
        })
        .await;

        if let Err(_) = result {
            error!("❌ Kafka unreachable, saving log locally.");
            kafka_repo.local_storage.save_log(&log);
        }
    }
}
