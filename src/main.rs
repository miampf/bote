use bote::commands;
use bote::config::get_app_directory;
use bote::logging;
use clap::{Parser, Subcommand};
use log::info;

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
    Init {},
    #[command(about = "Install a package")]
    Install {},
    #[command(about = "Import or create a library")]
    Library {
        #[command(subcommand)]
        command: LibraryCommands,
    },
    #[command(about = "Show a random silly pride flag :3")]
    Pride {},
    #[command(about = "Publish a package to a library")]
    Publish {
        #[arg(
            required = true,
            short,
            long,
            help = "The name of the library you want to publish to"
        )]
        library: String,
        #[arg(
            required = true,
            short,
            long,
            help = "The name your package should have in the library"
        )]
        name: String,
    },
    #[command(about = "Search your imported libraries for a package")]
    Search {},
    #[command(about = "Uninstall a package")]
    Uninstall {},
    #[command(about = "Upgrade installed packages")]
    Upgrade {},
}

#[derive(Subcommand)]
enum LibraryCommands {
    #[command(about = "Create a new library")]
    Create {
        #[arg(required = true, short, long, help = "The name of your new library")]
        name: String,
        #[arg(
            required = true,
            short,
            long,
            help = "Your name/the name you want to use to publish the library"
        )]
        owner_name: String,
    },
    #[command(about = "Add a new library to your local index")]
    Add {},
}

async fn run_subcommand(command: Commands) -> Result<(), anyhow::Error> {
    match command {
        Commands::Init {} => commands::init::run(),
        Commands::Install {} => commands::install::run(),
        Commands::Library { command } => match command {
            LibraryCommands::Add {} => todo!(),
            LibraryCommands::Create { name, owner_name } => {
                commands::library::create::run(name.as_str(), owner_name.as_str())
            }
        },
        Commands::Pride {} => commands::pride::run(),
        Commands::Publish { name, library } => commands::publish::run(name, library).await,
        Commands::Search {} => commands::search::run(),
        Commands::Uninstall {} => commands::uninstall::run(),
        Commands::Upgrade {} => commands::upgrade::run(),
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
