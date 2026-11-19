use super::*;

fn output(settings: &Settings) -> Result<String, Error> {
    let mut out = Vec::new();
    run(settings, &mut out)?;
    Ok(String::from_utf8(out).unwrap())
}

#[test]
fn prints_the_greeting() {
    assert_eq!(output(&Settings::default()).unwrap(), "Hello, world!\n");
}

#[test]
fn shouts_only_with_the_extra_feature() {
    let settings = Settings {
        shout: true,
        ..Settings::default()
    };
    let expected = if cfg!(feature = "extra") {
        "HELLO, WORLD!!\n"
    } else {
        "Hello, world!\n"
    };
    assert_eq!(output(&settings).unwrap(), expected);
}

#[test]
fn library_errors_pass_through() {
    let settings = Settings {
        name: "  ".to_owned(),
        ..Settings::default()
    };
    assert!(matches!(
        output(&settings),
        Err(Error::Greet(libskel::Error::EmptyName))
    ));
}
