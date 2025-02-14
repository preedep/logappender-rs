use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

/// Global circuit breaker for Kafka availability
pub static KAFKA_AVAILABLE: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(true)));

/// Periodically checks Kafka availability and updates the circuit breaker state
pub async fn monitor_kafka_health(kafka_broker: String) {
    loop {
        if let Err(_) = tokio::net::TcpStream::connect(&kafka_broker).await {
            KAFKA_AVAILABLE.store(false, Ordering::SeqCst);
            error!("⚠️ Kafka is unreachable. Switching to local storage...");
        } else {
            if !KAFKA_AVAILABLE.load(Ordering::SeqCst) {
                info!("✅ Kafka is back online! Resending failed logs...");
            }
            KAFKA_AVAILABLE.store(true, Ordering::SeqCst);
        }
        sleep(Duration::from_secs(10)).await;
    }
}
