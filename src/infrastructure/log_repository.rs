use async_trait::async_trait;
use crate::domain::log::LogMessage;

#[async_trait]
pub trait LogRepository {
    async fn save(&self, log: LogMessage);
}