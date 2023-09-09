use std::sync::Arc;
use std::time::SystemTime;

use bote::config::{config_callback, get_app_directory};
use fern::colors::ColoredLevelConfig;
use log::info;
use veilid_core::VeilidUpdate;

fn setup_logger_color_scheme() -> ColoredLevelConfig {
    use fern::colors::Color;
    ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .trace(Color::BrightBlack)
}

fn setup_stdio_logging() -> Result<fern::Dispatch, anyhow::Error> {
    let colors = setup_logger_color_scheme();

    let dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout());

    Ok(dispatch)
}

fn setup_file_logging() -> Result<fern::Dispatch, anyhow::Error> {
    let dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[Time: {}, Level: {}]\n\t{}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file(get_app_directory()? + "/bote.log")?);
    Ok(dispatch)
}

fn setup_logger() -> Result<(), anyhow::Error> {
    let dispatch = fern::Dispatch::new();

    dispatch
        .chain(setup_file_logging()?)
        .chain(setup_stdio_logging()?)
        .apply()?;

    Ok(())
}

fn update_callback(_update: VeilidUpdate) {}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_logger()?;

    let update_callback = Arc::new(update_callback);
    let config_callback = Arc::new(config_callback);
    let api = veilid_core::api_startup(update_callback, config_callback).await?;

    api.attach().await?;

    info!("Connected to veilid");

    api.detach().await?;
    api.shutdown().await;

    Ok(())
}
