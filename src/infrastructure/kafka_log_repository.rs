use crate::domain::log::LogMessage;
use crate::infrastructure::config::KafkaConfig;
use crate::infrastructure::local_log_storage::LocalLogStorage;
use crate::infrastructure::log_repository::LogRepository;
use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tracing::{error, info};

pub struct KafkaLogRepository {
    producer: FutureProducer,
    topic: String,
    pub local_storage: LocalLogStorage,
}

impl KafkaLogRepository {
    pub fn new(config: KafkaConfig, storage: LocalLogStorage) -> Self {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("acks", &config.acks)
            .set("enable.idempotence", &config.enable_idempotence.to_string())
            .set("retries", &config.retries.to_string())
            .set("message.timeout.ms", &config.message_timeout_ms.to_string())
            .set("security.protocol", &config.security_protocol)
            .set("sasl.mechanism", &config.sasl_mechanism)
            .set("sasl.username", &config.sasl_username)
            .set("sasl.password", &config.sasl_password)
            .set("ssl.ca.location", &config.ssl_ca_location)
            .create()
            .expect("Failed to create Kafka producer");

        Self {
            producer,
            topic: config.topic,
            local_storage: storage,
        }
    }

    pub async fn send_to_kafka(&self, log: &LogMessage) -> Result<(), String> {
        let payload = serde_json::to_string(&log).unwrap();
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&log.message);

        match self.producer.send(record, Duration::from_secs(1)).await {
            Ok(_) => {
                info!("✅ Log sent: {:?}", log);
                Ok(())
            }
            Err(e) => {
                error!("❌ Kafka send failed: {:?}", e);
                Err("Kafka send failed".to_string())
            }
        }
    }
}