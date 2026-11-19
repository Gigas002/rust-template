//! Optional extras, behind the `extra` feature: an example of a feature-gated module.

#[cfg(test)]
mod tests;

/// `greeting` in capitals, with the `!` doubled.
pub fn shout(greeting: &str) -> String {
    let mut loud = greeting.to_uppercase();
    if loud.ends_with('!') {
        loud.push('!');
    }
    loud
}
