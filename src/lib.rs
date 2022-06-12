mod cmd_dispatcher;
mod error;
#[cfg(test)]
mod tests;
pub(crate) mod parsers;

pub use cmd_dispatcher::{Command, CommandDispatcher};
pub use error::*;
