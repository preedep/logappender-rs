use crate::domain::log::{LogMessage, LogType};
use async_trait::async_trait;

#[async_trait]
pub trait LogService {
    async fn log(&self, log: LogMessage);
    fn supports_log_type(&self, log_type: LogType) -> bool;
}