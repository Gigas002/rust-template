//! libskel: the library half of the workspace. Domain logic only: no CLI parsing,
//! no config files, no logging setup (docs/ARCHITECTURE.md §1.1, §9).
//!
//! The greeting code is a placeholder showing the layout to copy: one folder per
//! module with its tests in a sibling `tests.rs`, a typed error, and an optional
//! module behind a Cargo feature.

mod error;
mod greeting;

#[cfg(feature = "extra")]
pub mod extra;

pub use error::Error;
pub use greeting::greet;
