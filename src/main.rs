mod application;
mod domain;
mod infrastructure;

use crate::domain::log::{LogBuilder};
use crate::infrastructure::async_log_queue::AsyncLogQueue;
use crate::infrastructure::background_log_resender;
use crate::infrastructure::circuit_breaker::monitor_kafka_health;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::kafka_log_repository::KafkaLogRepository;
use crate::infrastructure::local_log_storage::LocalLogStorage;
use log::info;
use std::sync::Arc;


use crate::domain::log::LogLevel::Info;
use crate::domain::log::LogType::AppLog;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let config = AppConfig::new();
    let local_storage = LocalLogStorage::new("logs_backup.json");
    let kafka_repo = Arc::new(KafkaLogRepository::new(
        config.get_kafka_config("default").unwrap(),
        local_storage,
    ));
    let log_queue = AsyncLogQueue::new(kafka_repo.clone());
    let bootstrap_server =  config.get_kafka_config("default").unwrap().bootstrap_servers;
    tokio::spawn(monitor_kafka_health(bootstrap_server));
    tokio::spawn(background_log_resender::resend_failed_logs(
        kafka_repo.clone(),
    ));


    for i in 1..=100 {
        let builder = LogBuilder::new(AppLog,Info, "Log message","1234").build();
        let log = builder.unwrap();
        log_queue.log(log).await;
    }

    info!("✅ Microservice log producer completed.");

    Ok(())
}
