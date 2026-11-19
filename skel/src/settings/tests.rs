use super::*;

fn file() -> FileConfig {
    FileConfig {
        name: Some("file-name".to_owned()),
        greeting: Some("Hi".to_owned()),
        shout: Some(true),
        log_level: Some(LogLevel::Info),
    }
}

#[test]
fn defaults_apply_when_nothing_is_set() {
    assert_eq!(resolve(&Cli::default(), None), Settings::default());
}

#[test]
fn the_config_file_overrides_the_defaults() {
    let settings = resolve(&Cli::default(), Some(file()));
    assert_eq!(settings.name, "file-name");
    assert_eq!(settings.greeting, "Hi");
    assert!(settings.shout);
    assert_eq!(settings.log_level, LogLevel::Info);
}

#[test]
fn the_command_line_overrides_the_config_file() {
    let cli = Cli {
        name: Some("cli-name".to_owned()),
        greeting: Some("Hey".to_owned()),
        log_level: Some(LogLevel::Trace),
        ..Cli::default()
    };
    let settings = resolve(&cli, Some(file()));
    assert_eq!(settings.name, "cli-name");
    assert_eq!(settings.greeting, "Hey");
    assert_eq!(settings.log_level, LogLevel::Trace);
}
