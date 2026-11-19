//! The settings resolver: CLI > config file > defaults, field by field
//! (docs/ARCHITECTURE.md §3). Everything after `main` takes a [`Settings`].

#[cfg(test)]
mod tests;

use serde::Deserialize;

use crate::{cli::Cli, config::FileConfig};

/// How much to log to stderr.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Nothing.
    Off,
    /// Failures.
    Error,
    /// Failures and suspicious input.
    #[default]
    Warn,
    /// What the program does.
    Info,
    /// How it does it.
    Debug,
    /// Everything.
    Trace,
}

/// The resolved settings: every field has a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    /// Who to greet.
    pub name: String,
    /// The greeting word.
    pub greeting: String,
    /// Print the greeting in capitals.
    pub shout: bool,
    /// How much to log to stderr.
    pub log_level: LogLevel,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            name: "world".to_owned(),
            greeting: "Hello".to_owned(),
            shout: false,
            log_level: LogLevel::default(),
        }
    }
}

/// Merges the command line and the config file over the defaults.
pub fn resolve(cli: &Cli, file: Option<FileConfig>) -> Settings {
    let file = file.unwrap_or_default();
    let defaults = Settings::default();
    Settings {
        name: cli.name.clone().or(file.name).unwrap_or(defaults.name),
        greeting: cli
            .greeting
            .clone()
            .or(file.greeting)
            .unwrap_or(defaults.greeting),
        // A flag can only turn shouting on; the config file can too.
        shout: cli.shout || file.shout.unwrap_or(defaults.shout),
        log_level: cli
            .log_level
            .or(file.log_level)
            .unwrap_or(defaults.log_level),
    }
}
