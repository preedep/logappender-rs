use serde::{Deserialize, Serialize};


#[derive(Default,Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LogType {
    #[default]
    AppLog,
    RequestLog,
    RequestExternalLog,
    PIILog,
    UserAccessLog
}
#[derive(Default,Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LogLevel {
    #[default]
    Info,
    Warning,
    Error,
    Debug
}
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogMessage {
    pub event_date_time: String,
    pub message: String,
    pub log_type: LogType,
    pub log_level: LogLevel,
    pub app_id: Option<String>,
    pub app_version: Option<String>,
    pub app_address: Option<String>,
    pub service_name: Option<String>,
    pub service_version: Option<String>,
    pub service_address: Option<String>,
}

impl LogMessage {
    pub fn new(log_type: LogType) -> LogMessage {
        let mut log_msg = LogMessage::default();
        log_msg.log_type = log_type;
        log_msg
    }
}



pub struct LogBuilder {
    log: LogMessage,
}
impl LogBuilder {
    pub fn new(log_type: LogType) -> LogBuilder {
        LogBuilder {
            log: LogMessage::new(log_type),
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
