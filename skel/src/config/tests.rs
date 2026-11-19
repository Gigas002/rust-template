use super::*;
use crate::{cli::Cli, settings};

#[test]
fn the_example_config_parses_and_documents_the_defaults() {
    // examples/config.toml shows every key with its default value; if a default changes,
    // the example must change with it.
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/config.toml");
    let file = load(&ConfigSource::Path(path.into())).unwrap().unwrap();
    assert!(file.name.is_some() && file.greeting.is_some());
    assert!(file.shout.is_some() && file.log_level.is_some());
    assert_eq!(
        settings::resolve(&Cli::default(), Some(file)),
        settings::Settings::default()
    );
}

#[test]
fn unknown_keys_are_errors_that_name_the_key() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(&path, "greting = \"Hi\"\n").unwrap();

    let err = load(&ConfigSource::Path(path.clone())).unwrap_err();
    let message = err.to_string();
    assert!(message.contains("greting"), "{message}");
    assert!(message.contains(&path.display().to_string()), "{message}");
}

#[test]
fn a_named_file_must_exist() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing.toml");
    assert!(matches!(
        load(&ConfigSource::Path(path)),
        Err(ConfigError::Read { .. })
    ));
}

#[test]
fn no_config_reads_nothing() {
    assert_eq!(load(&ConfigSource::None).unwrap(), None);
}
