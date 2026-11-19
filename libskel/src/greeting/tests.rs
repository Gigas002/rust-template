use super::*;

#[test]
fn greets_the_trimmed_name() {
    assert_eq!(greet("Hello", "  world ").unwrap(), "Hello, world!");
    assert_eq!(greet("Привет", "мир").unwrap(), "Привет, мир!");
}

#[test]
fn rejects_an_empty_name() {
    assert_eq!(greet("Hello", ""), Err(Error::EmptyName));
    assert_eq!(greet("Hello", " \t "), Err(Error::EmptyName));
}

#[test]
fn rejects_control_characters() {
    assert_eq!(
        greet("Hello", "a\u{1b}]0;title\u{7}"),
        Err(Error::ControlCharacter('\u{1b}'))
    );
}
