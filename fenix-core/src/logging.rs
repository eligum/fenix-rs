use chrono;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

/// Initializes the logging backend. After this, log output is sent to `stdout`
/// and the file specified by `path`.
pub fn setup(path: &str, level: LevelFilter) -> Result<(), fern::InitError> {
    let base_config = fern::Dispatch::new().level(level);

    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}]: {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(path)?);

    let palette = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::White);

    let term_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{}]: {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                palette.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout());

    base_config.chain(file_config).chain(term_config).apply()?;

    Ok(())
}
