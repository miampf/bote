use std::sync::Arc;

use log::{debug, info};
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

    loop {
        match api.get_state().await {
            Ok(veilid_state) => {
                if veilid_state.network.started && !veilid_state.network.peers.is_empty() {
                    info!(
                        "Network initialization done, we have {} peers!",
                        veilid_state.network.peers.len()
                    );
                    break;
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(api)
}
