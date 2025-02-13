use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use async_trait::async_trait;
use crate::domain::log::LogMessage;
use crate::infrastructure::log_repository::LogRepository;
use crate::infrastructure::config::KafkaConfig;
use tracing::{info, error};

pub struct KafkaLogRepository {
    producer: FutureProducer,
    topic: String,
}

impl KafkaLogRepository {
    pub fn new(config: KafkaConfig) -> Self {
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
        }
    }
}

#[async_trait]
impl LogRepository for KafkaLogRepository {
    async fn save(&self, log: LogMessage) {
        let payload = serde_json::to_string(&log).unwrap();
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&log.message);

        match self.producer.send(record, Duration::from_secs(1)).await {
            Ok(_) => info!("✅ Sent log: {:?}", log),
            Err(e) => error!("❌ Failed to send log: {:?}", e),
        }
    }
}