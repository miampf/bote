use bote::commands;
use bote::config::get_app_directory;
use bote::logging;
use clap::{Parser, Subcommand};
use log::info;
use veilid_core::VeilidUpdate;

#[derive(Parser)]
#[command(
    author = "miampf <miampf@proton.me>",
    about = "A package manager build upon the veilid network"
)]
struct Cli {
    #[arg(long, help = "Delete the old log file")]
    clear_log_file: bool,
    #[arg(short, help = "Increase the verbosity of the output (maximum is -vvv)", action = clap::ArgAction::Count)]
    verbosity: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize bote")]
    Init,
    #[command(about = "Install a package")]
    Install,
    #[command(about = "Import or create a library")]
    Library,
    #[command(about = "Show a random silly pride flag :3")]
    Pride,
    #[command(about = "Publish a package to a library")]
    Publish,
    #[command(about = "Search your imported libraries for a package")]
    Search,
    #[command(about = "Uninstall a package")]
    Uninstall,
    #[command(about = "Upgrade installed packages")]
    Upgrade,
}

async fn run_subcommand(command: Commands) -> Result<(), anyhow::Error> {
    match command {
        Commands::Init => commands::init::run(),
        Commands::Install => commands::install::run(),
        Commands::Library => commands::library::run(),
        Commands::Pride => commands::pride::run(),
        Commands::Publish => commands::publish::run().await,
        Commands::Search => commands::search::run(),
        Commands::Uninstall => commands::uninstall::run(),
        Commands::Upgrade => commands::upgrade::run(),
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    logging::setup_logger(cli.verbosity)?;

    if cli.clear_log_file {
        std::fs::remove_file(get_app_directory()? + "/bote.log")?;
    }

    if let Some(command) = cli.command {
        run_subcommand(command).await?;
    }

    info!("Disconnected from veilid");

    Ok(())
}
