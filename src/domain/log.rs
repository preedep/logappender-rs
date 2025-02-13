use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogMessage {
    pub message: String,
}
pub struct LogBuilder {
    log: LogMessage,
}
impl LogBuilder {
    pub fn new() -> LogBuilder {
        LogBuilder {
            log: LogMessage::new(),
        }
    }
    pub fn message(mut self, message: String) -> LogBuilder {
        self.log.message = message;
        self
    }
    pub fn build(&self) -> LogMessage {
        self.log.clone()
    }
}
impl LogMessage {
    pub fn new() -> LogMessage {
        LogMessage::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_builder_creates_log_with_message() {
        let log = LogBuilder::new()
            .message("Test message".to_string())
            .build();
        assert_eq!(log.message, "Test message");
    }

    #[test]
    fn log_builder_creates_default_log() {
        let log = LogBuilder::new().build();
        assert_eq!(log.message, "");
    }

    #[test]
    fn log_builder_allows_message_update() {
        let log = LogBuilder::new()
            .message("Initial message".to_string())
            .message("Updated message".to_string())
            .build();
        assert_eq!(log.message, "Updated message");
    }

    #[test]
    fn log_default_is_empty_message() {
        let log = LogMessage::default();
        assert_eq!(log.message, "");
    }
}
