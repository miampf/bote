use crate::config::get_app_directory;
use fern::colors::Color;
use fern::colors::ColoredLevelConfig;
use std::time::SystemTime;

fn setup_logger_color_scheme() -> ColoredLevelConfig {
    ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .trace(Color::BrightBlack)
}

fn setup_stdio_logging(verbosity: u8) -> Result<fern::Dispatch, anyhow::Error> {
    let colors = setup_logger_color_scheme();

    let filter = match verbosity {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}: {} {}] {}",
                record.target(),
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                message
            ))
        })
        .level(filter)
        .chain(std::io::stdout());

    Ok(dispatch)
}

fn setup_file_logging() -> Result<fern::Dispatch, anyhow::Error> {
    let dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[Time: {}, Level: {}, From: {}]\n\t{}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(fern::log_file(get_app_directory()? + "/bote.log")?);
    Ok(dispatch)
}

pub fn setup_logger(verbosity: u8) -> Result<(), anyhow::Error> {
    let verbosity = verbosity.clamp(0, 3);

    let dispatch = fern::Dispatch::new();

    dispatch
        .chain(setup_file_logging()?)
        .chain(setup_stdio_logging(verbosity)?)
        .apply()?;

    Ok(())
}
