use crate::infrastructure::kafka_log_repository::KafkaLogRepository;
use tokio::time::{sleep, Duration};
use tracing::{info, error};
use std::sync::Arc;
use crate::infrastructure::circuit_breaker::KAFKA_AVAILABLE;
use std::sync::atomic::Ordering;

pub async fn resend_failed_logs(kafka_repo: Arc<KafkaLogRepository>) {
    loop {
        if !KAFKA_AVAILABLE.load(Ordering::SeqCst) {
            sleep(Duration::from_secs(30)).await; // Wait before checking again
            continue;
        }

        let logs = kafka_repo.local_storage.fetch_unsent_logs();
        if logs.is_empty() {
            sleep(Duration::from_secs(30)).await;
            continue;
        }

        let mut successfully_sent = Vec::new();

        for log in logs {
            match kafka_repo.send_to_kafka(&log).await {
                Ok(_) => {
                    info!("✅ Successfully resent log: {:?}", log);
                    successfully_sent.push(log);
                }
                Err(_) => error!("❌ Failed to resend log: {:?}", log),
            }
        }

        kafka_repo.local_storage.clear_sent_logs(&successfully_sent);
        sleep(Duration::from_secs(10)).await;
    }
}