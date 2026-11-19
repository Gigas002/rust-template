use super::*;

#[test]
fn shouts() {
    assert_eq!(shout("Hello, world!"), "HELLO, WORLD!!");
    assert_eq!(shout("hi"), "HI");
}
