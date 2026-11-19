//! Command-line arguments. Parsed once, in `main`; nothing below `settings` sees them.

#[cfg(test)]
mod tests;

use std::path::PathBuf;

use crate::{config::ConfigSource, settings::LogLevel};

/// skel greets someone. The values given here override the config file.
#[derive(Debug, Default, clap::Parser)]
#[command(version, about)]
pub struct Cli {
    /// Who to greet [default: the config file's `name`, else "world"].
    pub name: Option<String>,

    /// The greeting word [default: the config file's `greeting`, else "Hello"].
    #[arg(long, short)]
    pub greeting: Option<String>,

    /// Print the greeting in capitals (needs the `extra` feature).
    #[arg(long)]
    pub shout: bool,

    /// Read this config file instead of the default one.
    #[arg(long, short, value_name = "PATH", conflicts_with = "no_config")]
    pub config: Option<PathBuf>,

    /// Don't read any config file.
    #[arg(long)]
    pub no_config: bool,

    /// How much to log to stderr [default: the config file's `log_level`, else "warn"].
    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,
}

impl Cli {
    /// Which config file to read, from `--config` and `--no-config`.
    pub fn config_source(&self) -> ConfigSource {
        match (&self.config, self.no_config) {
            (_, true) => ConfigSource::None,
            (Some(path), false) => ConfigSource::Path(path.clone()),
            (None, false) => ConfigSource::Default,
        }
    }
}
