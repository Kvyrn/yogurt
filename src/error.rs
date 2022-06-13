use crate::InvalidCommandReason::UnknownCommand;
use nom::Err;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidCommand(InvalidCommandReason),
    ExecutionFailed,
    NotACommand,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Err<nom::error::Error<&str>>> for Error {
    fn from(_: Err<nom::error::Error<&str>>) -> Self {
        Error::InvalidCommand(UnknownCommand)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum InvalidCommandReason {
    UnknownCommand,
    MissingArgument,
    InvalidArgument,
    UnclosedQuote,
    Unauthorised,
}
