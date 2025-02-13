use serde::{Deserialize, Serialize};

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Log {
    pub message: String
}
pub struct LogBuilder {
    log: Log
}
impl LogBuilder{
    pub fn new() -> LogBuilder {
        LogBuilder {
            log: Log::new()
        }
    }
    pub fn message(mut self, message: String) -> LogBuilder {
        self.log.message = message;
        self
    }
    pub fn build(&self) -> Log {
        self.log.clone()
    }
}
impl Log {
    pub fn new() -> Log {
        Log::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_builder_creates_log_with_message() {
        let log = LogBuilder::new().message("Test message".to_string()).build();
        assert_eq!(log.message, "Test message");
    }

    #[test]
    fn log_builder_creates_default_log() {
        let log = LogBuilder::new().build();
        assert_eq!(log.message, "");
    }

    #[test]
    fn log_builder_allows_message_update() {
        let log = LogBuilder::new().message("Initial message".to_string()).message("Updated message".to_string()).build();
        assert_eq!(log.message, "Updated message");
    }

    #[test]
    fn log_default_is_empty_message() {
        let log = Log::default();
        assert_eq!(log.message, "");
    }
}