use crate::domain::log::{LogBuilder, LogLevel, LogMessage, LogType, RequestObj, ResponseObj};
use crate::infrastructure::async_log_queue::AsyncLogQueue;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::kafka_log_repository::KafkaLogRepository;
use crate::infrastructure::local_log_storage::LocalLogStorage;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use crate::infrastructure::background_log_resender;
use crate::infrastructure::circuit_breaker::monitor_kafka_health;

pub struct LogService {
    pub config: AppConfig,
    pub queue_table: Option<HashMap<LogType, Arc<AsyncLogQueue>>>,
}

impl LogService {
    pub fn new(config: AppConfig) -> Self {
        let mut queue_table = HashMap::new();
        for (name, kafka_config) in config.clone().kafka {
            let local_storage = LocalLogStorage::new(&format!("logs_backup_{}.json", name));
            let kafka_repo = Arc::new(KafkaLogRepository::new(kafka_config, local_storage));
            queue_table.insert(LogType::AppLog, Arc::new(AsyncLogQueue::new(kafka_repo)));
            //queue_table.insert(LogType::RequestLog, Arc::new(AsyncLogQueue::new(kafka_repo)));
            //queue_table.insert(LogType::RequestExternalLog, Arc::new(AsyncLogQueue::new(kafka_repo)));
        }
        let bootstrap_server = config
            .get_kafka_config("default")
            .unwrap()
            .bootstrap_servers;
        tokio::spawn(monitor_kafka_health(bootstrap_server));

        Self {
            config: config.clone(),
            queue_table: Some(queue_table),
        }
    }
    pub async fn resend_if_failed(&self) {
        for (name, kafka_config) in self.config.clone().kafka {
            let kafka_repo = Arc::new(KafkaLogRepository::new(kafka_config, LocalLogStorage::new(&format!("logs_backup_{}.json", name))));
            tokio::spawn(background_log_resender::resend_failed_logs(kafka_repo.clone()));
        }
    }

    pub async fn log_app(&self, log_level: LogLevel, message: &str, correlation_id: &str) {
        let log_msg = LogBuilder::new(LogType::AppLog, log_level, message, correlation_id)
            .build()
            .unwrap();

        self.log(log_msg).await;
    }

    pub async fn log_request(
        &self,
        log_level: LogLevel,
        message: &str,
        correlation_id: &str,
        request: RequestObj,
        response: ResponseObj,
    ) {
        let log_msg = LogBuilder::new(LogType::RequestLog, log_level, message, correlation_id)
            .request(request)
            .response(response)
            .build()
            .unwrap();
        self.log(log_msg).await;
    }

    pub async fn log_external_request(
        &self,
        log_level: LogLevel,
        message: &str,
        correlation_id: &str,
        request: RequestObj,
        response: ResponseObj,
    ) {
        let log_msg = LogBuilder::new(
            LogType::RequestExternalLog,
            log_level,
            message,
            correlation_id,
        )
        .request(request)
        .response(response)
        .build()
        .unwrap();
        self.log(log_msg).await;
    }

    pub async fn log(&self, log: LogMessage) {
        debug!("Logging message: {:?}", log);
        if let Some(queue_table) = &self.queue_table {
            if let Some(queue) = queue_table.get(&log.log_type) {
                queue.log(log).await;
            }
        }
    }
}
