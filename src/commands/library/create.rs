use crate::veilid;

/// run() runs the create subcommand.
pub async fn run(name: &str, owner_name: &str) -> Result<(), anyhow::Error> {
    let veilid_api = veilid::connect_to_veilid().await?;
    let routing = veilid_api.routing_context().with_privacy()?;

    Ok(())
}
