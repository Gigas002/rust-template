//! The config file: find it, read it, deserialize it. Precedence is not applied here
//! (that's `settings`); this module only turns TOML into [`FileConfig`].

#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::settings::LogLevel;

/// `config.toml`, as written. Every field is optional; unknown keys are errors, so a
/// typo fails loudly instead of being ignored. `examples/config.toml` documents it.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileConfig {
    /// Who to greet.
    pub name: Option<String>,
    /// The greeting word.
    pub greeting: Option<String>,
    /// Print the greeting in capitals.
    pub shout: Option<bool>,
    /// How much to log to stderr.
    pub log_level: Option<LogLevel>,
}

/// Where the config comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    /// The default location, read only if it exists.
    Default,
    /// A path the user named, which must exist.
    Path(PathBuf),
    /// No config file.
    None,
}

/// Why the config file couldn't be used. Each message names the file.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// The file couldn't be read.
    #[error("cannot read {}: {source}", path.display())]
    Read {
        /// The config file.
        path: PathBuf,
        /// What went wrong.
        source: std::io::Error,
    },
    /// The file isn't valid TOML, or has an unknown key or a wrong type.
    #[error("invalid config {}: {source}", path.display())]
    Parse {
        /// The config file.
        path: PathBuf,
        /// toml's message, with the line and the key.
        source: toml::de::Error,
    },
}

/// `<config dir>/skel/config.toml`: `~/.config/skel/config.toml` on Linux.
pub fn default_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("skel").join("config.toml"))
}

/// Loads the config from `source`. `Ok(None)` when there is none to read.
pub fn load(source: &ConfigSource) -> Result<Option<FileConfig>, ConfigError> {
    match source {
        ConfigSource::None => Ok(None),
        ConfigSource::Path(path) => read(path).map(Some),
        ConfigSource::Default => match default_path() {
            Some(path) if path.is_file() => read(&path).map(Some),
            _ => Ok(None),
        },
    }
}

fn read(path: &Path) -> Result<FileConfig, ConfigError> {
    let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.to_owned(),
        source,
    })?;
    toml::from_str(&text).map_err(|source| ConfigError::Parse {
        path: path.to_owned(),
        source,
    })
}
