use clap::{CommandFactory as _, Parser as _};

use super::*;

#[test]
fn the_command_definition_is_valid() {
    Cli::command().debug_assert();
}

#[test]
fn parses_every_option() {
    let cli = Cli::try_parse_from([
        "skel",
        "Ann",
        "-g",
        "Hi",
        "--shout",
        "-c",
        "x.toml",
        "--log-level",
        "debug",
    ])
    .unwrap();
    assert_eq!(cli.name.as_deref(), Some("Ann"));
    assert_eq!(cli.greeting.as_deref(), Some("Hi"));
    assert!(cli.shout);
    assert_eq!(cli.config_source(), ConfigSource::Path("x.toml".into()));
    assert_eq!(cli.log_level, Some(LogLevel::Debug));
}

#[test]
fn config_source_follows_the_flags() {
    assert_eq!(Cli::default().config_source(), ConfigSource::Default);
    let cli = Cli::try_parse_from(["skel", "--no-config"]).unwrap();
    assert_eq!(cli.config_source(), ConfigSource::None);
    assert!(Cli::try_parse_from(["skel", "--no-config", "-c", "x.toml"]).is_err());
}
