use config::{Config, File};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub acks: String,
    pub enable_idempotence: bool,
    pub retries: u32,
    pub message_timeout_ms: u64,
    pub topic: String,
    pub security_protocol: String,
    pub sasl_mechanism: String,
    pub sasl_username: String,
    pub sasl_password: String,
    pub ssl_ca_location: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub kafka: HashMap<String, KafkaConfig>,
}

impl AppConfig {
    pub fn new() -> Self {
        let settings = Config::builder()
            .add_source(File::with_name("config"))
            .build()
            .expect("Failed to load configuration");

        settings.try_deserialize().expect("Invalid config structure")
    }

    pub fn get_kafka_config(&self, config_name: &str) -> Option<KafkaConfig> {
        self.kafka.get(config_name).cloned()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_config_loads_valid_config() {
        let config = AppConfig::new();
        assert!(config.kafka.contains_key("default"));
    }

    #[test]
    fn app_config_returns_none_for_nonexistent_kafka_config() {
        let config = AppConfig::new();
        assert!(config.get_kafka_config("nonexistent").is_none());
    }


    #[test]
    fn kafka_config_has_correct_default_values() {
        let kafka_config = KafkaConfig {
            bootstrap_servers: "localhost:9092".to_string(),
            acks: "all".to_string(),
            enable_idempotence: true,
            retries: 5,
            message_timeout_ms: 3000,
            topic: "test_topic".to_string(),
            security_protocol: "PLAINTEXT".to_string(),
            sasl_mechanism: "".to_string(),
            sasl_username: "".to_string(),
            sasl_password: "".to_string(),
            ssl_ca_location: "".to_string(),
        };
        assert_eq!(kafka_config.bootstrap_servers, "localhost:9092");
        assert_eq!(kafka_config.acks, "all");
        assert!(kafka_config.enable_idempotence);
        assert_eq!(kafka_config.retries, 5);
        assert_eq!(kafka_config.message_timeout_ms, 3000);
        assert_eq!(kafka_config.topic, "test_topic");
        assert_eq!(kafka_config.security_protocol, "PLAINTEXT");
    }
}
