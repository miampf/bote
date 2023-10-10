use core::time;

use crate::db;
use crate::veilid;

use log::{debug, info};
use serde::{Deserialize, Serialize};
use veilid_core::DHTSchema;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct DHTLibraryHeader<'a> {
    library_name: &'a str,
    library_owner: &'a str,
    pkg_count: u16,
}

/// run() runs the create subcommand.
pub async fn run(name: &str, owner_name: &str) -> Result<(), anyhow::Error> {
    let veilid_api = veilid::connect_to_veilid().await?;
    let routing = veilid_api.routing_context().with_privacy()?;

    let dht = routing
        .create_dht_record(DHTSchema::dflt(u16::MAX), None)
        .await?;
    info!("Created DHT with key {}", dht.key());

    let lib_header = DHTLibraryHeader {
        library_name: name,
        library_owner: owner_name,
        pkg_count: 0,
    };
    routing
        .set_dht_value(
            *dht.key(),
            0,
            serde_json::to_string(&lib_header)?.into_bytes(),
        )
        .await?;
    debug!("Wrote library header");

    // TEST: This is just for reading stuff
    // and not writing to veilid unnecessarily.
    // ---------------------------------------
    let lib_header_json = routing.get_dht_value(*dht.key(), 0, false).await?;
    assert!(lib_header_json.is_some());
    let bin_data = lib_header_json.unwrap().data().to_vec();
    let json_string = String::from_utf8(bin_data)?;
    let lib_header_parsed = serde_json::from_str(json_string.as_str())?;
    info!("{:?}", lib_header_parsed);
    // --------------------------------------

    let db = db::open_db().await?;
    db::write_table_to_db(&db, name, owner_name, dht.key().to_string().as_str(), true).await?;
    info!("Successfully created library");

    std::thread::sleep(time::Duration::from_secs(3));

    db.close().await;

    veilid_api.detach().await?;
    veilid_api.shutdown().await;

    Ok(())
}
