mod domain;
mod application;
mod infrastructure;

use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    info!("Hello, world!");
    Ok(())
}
