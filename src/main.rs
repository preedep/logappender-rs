mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;
use std::time::Duration;
use log::info;
use tokio::time::sleep;
use crate::domain::log::{LogBuilder, LogMessage};
use crate::infrastructure::async_log_queue::AsyncLogQueue;
use crate::infrastructure::background_log_resender;
use crate::infrastructure::circuit_breaker::monitor_kafka_health;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::kafka_log_repository::KafkaLogRepository;
use crate::infrastructure::local_log_storage::LocalLogStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let config = AppConfig::new();
    let local_storage = LocalLogStorage::new("logs_backup.json");
    let kafka_repo = Arc::new(KafkaLogRepository::new(config.get_kafka_config("default").unwrap(), local_storage));
    let log_queue = AsyncLogQueue::new(kafka_repo.clone());

    tokio::spawn(monitor_kafka_health("localhost:9092".to_string()));
    tokio::spawn(background_log_resender::resend_failed_logs(kafka_repo.clone()));

    for i in 1..=100 {
        let builder = LogBuilder::new().message(format!("Log message #{}", i));
        let log = builder.build();
        log_queue.log(log).await;
        sleep(Duration::from_millis(10)).await;
    }

    info!("✅ Microservice log producer completed.");

    Ok(())
}
