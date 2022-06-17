#![allow(clippy::redundant_field_names)]

pub mod argument;
pub mod dispatcher;
mod error;
pub(crate) mod parsers;
#[cfg(test)]
mod tests;

pub use dispatcher::*;
pub use error::*;
