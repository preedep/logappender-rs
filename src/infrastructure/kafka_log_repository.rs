use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use async_trait::async_trait;
use crate::domain::log::LogMessage;
use crate::infrastructure::log_repository::LogRepository;
use crate::infrastructure::config::KafkaConfig;
use crate::infrastructure::local_log_storage::LocalLogStorage;
use crate::infrastructure::circuit_breaker::KAFKA_AVAILABLE;
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use tokio_retry::Retry;
use tracing::{info, error};
use std::sync::atomic::Ordering;

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
        if !KAFKA_AVAILABLE.load(Ordering::SeqCst) {
            error!("⚠️ Kafka unavailable. Saving log to local storage.");
            self.local_storage.save_log(log);
            return Err("Kafka unavailable".to_string());
        }

        let payload = serde_json::to_string(&log).unwrap();
        let topic = self.topic.clone(); // Capture the topic name
        let key = log.message.clone(); // Capture the key

        let retry_strategy = ExponentialBackoff::from_millis(500)
            .map(jitter)
            .take(5); // Retry up to 5 times

        let result = Retry::spawn(retry_strategy, || async {
            // Create a new FutureRecord inside the retry logic
            let record = FutureRecord::to(&topic)
                .payload(&payload)
                .key(&key);

            self.producer.send(record, Duration::from_secs(1)).await
        }).await;

        match result {
            Ok(_) => {
                info!("✅ Log sent: {:?}", log);
                Ok(())
            }
            Err(err) => {
                error!("❌ Kafka send failed: {:?}. Saving log to local storage.", err);
                self.local_storage.save_log(log);
                Err("Kafka send failed".to_string())
            }
        }
    }

}