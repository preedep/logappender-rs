use crate::domain::log::LogMessage;
use crate::infrastructure::log_repository::LogRepository;
use async_trait::async_trait;

#[async_trait]
pub trait LogService {
    async fn log(&self, log: LogMessage);
}

pub struct LogServiceImpl<R: LogRepository + Sync + Send> {
    repository: R,
}

impl<R: LogRepository + Sync + Send> LogServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: LogRepository + Sync + Send> LogService for LogServiceImpl<R> {
    async fn log(&self, log: LogMessage) {
        self.repository.save(log).await;
    }
}
