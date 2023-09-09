use std::sync::Arc;

use bote::config::config_callback;
use bote::logging;
use log::info;
use veilid_core::VeilidUpdate;

fn update_callback(_update: VeilidUpdate) {}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    logging::setup_logger()?;

    let update_callback = Arc::new(update_callback);
    let config_callback = Arc::new(config_callback);
    let api = veilid_core::api_startup(update_callback, config_callback).await?;

    api.attach().await?;

    info!("Connected to veilid");

    api.detach().await?;
    api.shutdown().await;

    info!("Disconnected from veilid");

    Ok(())
}
