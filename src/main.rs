use log::info;
use crate::application::log_service::LogService;
use crate::domain::log::LogLevel;
use crate::infrastructure::config::AppConfig;

mod application;
mod domain;
mod infrastructure;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let config = AppConfig::new();
    let log_service = LogService::new(config.clone());
    log_service.resend_if_failed().await;

    for _ in 0..10 {
        log_service
            .log_app(LogLevel::Info,
                     "Microservice log producer is running.",
                     "123"
            )
            .await;


        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }


    info!("✅ Microservice log producer completed.");
    Ok(())
}
