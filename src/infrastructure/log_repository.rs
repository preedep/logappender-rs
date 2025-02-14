use crate::domain::log::LogMessage;
use async_trait::async_trait;

#[async_trait]
pub trait LogRepository: Send + Sync {
    async fn save(&self, log: &LogMessage);
}