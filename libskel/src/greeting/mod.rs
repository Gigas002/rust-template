//! Building greetings.

#[cfg(test)]
mod tests;

use crate::Error;

/// Greets `name` with `greeting`: `greet("Hello", "world")` is `"Hello, world!"`.
///
/// `name` is trimmed. It must not be empty, and must not hold control characters: text
/// that ends up on a terminal is checked where it enters the program.
///
/// # Errors
///
/// [`Error::EmptyName`] or [`Error::ControlCharacter`].
pub fn greet(greeting: &str, name: &str) -> Result<String, Error> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::EmptyName);
    }
    if let Some(c) = name.chars().find(|c| c.is_control()) {
        return Err(Error::ControlCharacter(c));
    }
    tracing::debug!(name, "greeting");
    Ok(format!("{greeting}, {name}!"))
}
