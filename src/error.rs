use std::fmt::{Display, Formatter};
use crate::InvalidCommandReason::UnknownCommand;
use nom::Err;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidCommand(InvalidCommandReason),
    ExecutionFailed,
    NotACommand,
    IncompleteBuilder,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Err<nom::error::Error<&str>>> for Error {
    fn from(_: Err<nom::error::Error<&str>>) -> Self {
        Error::InvalidCommand(UnknownCommand)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Eq, PartialEq)]
pub enum InvalidCommandReason {
    UnknownCommand,
    MissingArgument,
    InvalidArgument,
    UnknownArgument,
    Unauthorised,
}
