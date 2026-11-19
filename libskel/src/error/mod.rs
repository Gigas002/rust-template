//! The library's error type.

#[cfg(test)]
mod tests;

/// Everything the library can fail with. One variant per failure a caller can act on;
/// each message says what was wrong with which input.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// A name was empty or only whitespace.
    #[error("the name is empty")]
    EmptyName,
    /// A name had a control character, which would reach the terminal as-is.
    #[error("the name contains a control character ({0:?})")]
    ControlCharacter(char),
}
