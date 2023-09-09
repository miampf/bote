use std::sync::Arc;

use bote::config::{config_callback, get_app_directory};
use bote::logging;
use clap::{Parser, Subcommand};
use log::info;
use veilid_core::VeilidUpdate;

#[derive(Parser)]
#[command(
    author = "miampf <miampf@proton.me>",
    about = "A package manager build upon the veilid network."
)]
struct Cli {
    #[arg(long, help = "Delete the old log file")]
    clear_log_file: bool,
    #[arg(short, help = "Increase the verbosity of the output (maximum is -vvv)", action = clap::ArgAction::Count)]
    verbosity: u8,
}

/// update_callback() is called every time if something interesting happens with veilid.
fn update_callback(_update: VeilidUpdate) {}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    logging::setup_logger(cli.verbosity)?;

    if cli.clear_log_file {
        std::fs::remove_file(get_app_directory()? + "/bote.log")?;
    }

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
