//! The application: takes [`Settings`], does the work, writes the output. It never sees
//! the command line or the config file.

#[cfg(test)]
mod tests;

use std::io::Write;

use crate::settings::Settings;

/// Why a run failed.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The library refused the input.
    #[error(transparent)]
    Greet(#[from] libskel::Error),
    /// Writing the output failed.
    #[error("cannot write the output: {0}")]
    Io(#[from] std::io::Error),
}

/// Runs the app, writing its output to `out` (stdout in `main`, a buffer in tests).
pub fn run(settings: &Settings, out: &mut impl Write) -> Result<(), Error> {
    let greeting = libskel::greet(&settings.greeting, &settings.name)?;

    #[cfg(feature = "extra")]
    let greeting = if settings.shout {
        libskel::extra::shout(&greeting)
    } else {
        greeting
    };
    #[cfg(not(feature = "extra"))]
    if settings.shout {
        tracing::warn!("built without the `extra` feature; not shouting");
    }

    writeln!(out, "{greeting}")?;
    Ok(())
}
