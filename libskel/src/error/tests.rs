use super::*;

#[test]
fn messages_name_the_problem() {
    assert_eq!(Error::EmptyName.to_string(), "the name is empty");
    assert_eq!(
        Error::ControlCharacter('\u{1b}').to_string(),
        "the name contains a control character ('\\u{1b}')"
    );
}
