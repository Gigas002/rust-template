//! Logging setup: one `tracing` subscriber writing to stderr, so stdout stays for
//! output (docs/ARCHITECTURE.md §5). Libraries only emit events.

#[cfg(test)]
mod tests;

use tracing_subscriber::filter::LevelFilter;

use crate::settings::LogLevel;

/// Installs the subscriber. Call once, right after the settings are resolved.
pub fn init(level: LogLevel) {
    tracing_subscriber::fmt()
        .with_max_level(level_filter(level))
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();
}

fn level_filter(level: LogLevel) -> LevelFilter {
    match level {
        LogLevel::Off => LevelFilter::OFF,
        LogLevel::Error => LevelFilter::ERROR,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Trace => LevelFilter::TRACE,
    }
}
