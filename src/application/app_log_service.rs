use crate::application::log_service::LogService;
use crate::domain::log::{LogMessage, LogType};
use crate::infrastructure::log_repository::LogRepository;
use async_trait::async_trait;

pub struct AppLogService<R: LogRepository> {
    repository: R,
}

impl<R: LogRepository> AppLogService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: LogRepository> LogService for AppLogService<R> {
    async fn log(&self, log: LogMessage) {
        if self.supports_log_type(log.log_type.clone()) {
            self.repository.save(&log).await;
        }
    }

    fn supports_log_type(&self, log_type: LogType) -> bool {
        matches!(log_type, LogType::AppLog)
    }
}