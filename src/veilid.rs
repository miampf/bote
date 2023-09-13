use std::sync::Arc;

use log::debug;
use veilid_core::{VeilidAPI, VeilidUpdate};

use crate::config::config_callback;

fn update_callback(update: VeilidUpdate) {
    debug!("Update from veilid: {:?}", update);
}

/// connect_to_veilid() connects to the veilid network and returns the resulting API.
pub async fn connect_to_veilid() -> Result<VeilidAPI, anyhow::Error> {
    let update_callback = Arc::new(update_callback);
    let config_callback = Arc::new(config_callback);
    let api = veilid_core::api_startup(update_callback, config_callback).await?;

    api.attach().await?;

    Ok(api)
}
