//! skel: the binary half of the workspace.
//!
//! `main` only wires things together (docs/ARCHITECTURE.md §1.2): parse the command
//! line, load the config file, resolve [`settings::Settings`], start logging, run the
//! app, and turn the outcome into an exit code.

mod app;
mod cli;
mod config;
mod logger;
mod settings;

use std::process::ExitCode;

use clap::Parser as _;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    let file = match config::load(&cli.config_source()) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::from(2);
        }
    };
    let settings = settings::resolve(&cli, file);
    logger::init(settings.log_level);

    match app::run(&settings, &mut std::io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        // `skel | head -0`: the reader is gone, which isn't our failure.
        Err(app::Error::Io(err)) if err.kind() == std::io::ErrorKind::BrokenPipe => {
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
