//! End-to-end tests: run the real binary. Every run gets its own temporary HOME and
//! XDG dirs, so no test reads or writes the user's real config (docs/ARCHITECTURE.md §6.3).

use std::{
    path::Path,
    process::{Command, Output, Stdio},
};

/// Runs skel with `home` as its HOME and XDG config dir and nothing else in its environment.
fn skel(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_skel"))
        .args(args)
        .env_clear()
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .output()
        .expect("the skel binary runs")
}

fn stdout(output: &Output) -> &str {
    std::str::from_utf8(&output.stdout).unwrap()
}

fn stderr(output: &Output) -> &str {
    std::str::from_utf8(&output.stderr).unwrap()
}

#[test]
fn greets_with_the_defaults() {
    let home = tempfile::tempdir().unwrap();
    let output = skel(home.path(), &["--no-config"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "Hello, world!\n");
}

#[test]
fn the_command_line_overrides_the_config_file() {
    let home = tempfile::tempdir().unwrap();
    let config = home.path().join("custom.toml");
    std::fs::write(&config, "name = \"file\"\ngreeting = \"Hi\"\n").unwrap();

    let output = skel(home.path(), &["--config", config.to_str().unwrap(), "cli"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "Hi, cli!\n");
}

#[cfg(target_os = "linux")]
#[test]
fn reads_the_default_config_location() {
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join(".config/skel");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("config.toml"), "greeting = \"Howdy\"\n").unwrap();

    let output = skel(home.path(), &[]);
    assert_eq!(stdout(&output), "Howdy, world!\n", "{}", stderr(&output));
}

#[test]
fn an_unknown_config_key_exits_2_and_is_named() {
    let home = tempfile::tempdir().unwrap();
    let config = home.path().join("config.toml");
    std::fs::write(&config, "colour = \"red\"\n").unwrap();

    let output = skel(home.path(), &["--config", config.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).contains("colour"), "{}", stderr(&output));
    assert!(stdout(&output).is_empty());
}

#[test]
fn invalid_input_exits_1_with_a_message() {
    let home = tempfile::tempdir().unwrap();
    let output = skel(home.path(), &["--no-config", "   "]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stderr(&output), "error: the name is empty\n");
}

#[test]
fn a_closed_stdout_is_not_an_error() {
    // `skel | head -0`: whether skel writes before or after the reader is gone, it exits 0.
    let home = tempfile::tempdir().unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_skel"))
        .arg("--no-config")
        .env_clear()
        .env("HOME", home.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", stderr(&output));
}
