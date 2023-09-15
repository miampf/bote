use log::info;

use crate::veilid::connect_to_veilid;

/// run() runs the publish subcommand which is used to publish a package to a library.
pub async fn run(name: String, library: String) -> Result<(), anyhow::Error> {
    // TODO: Implement a publishing mechanism

    info!("Connecting to the veilid network");
    let veilid_api = connect_to_veilid().await?;

    Ok(())
}
