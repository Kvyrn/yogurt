use crate::{Error, InvalidCommandReason, Result};

pub trait Argument {
    type Output;

    fn parse(token: String) -> Result<Self::Output>;
}

pub struct StringArgument;

impl Argument for StringArgument {
    type Output = String;

    fn parse(token: String) -> Result<Self::Output> {
        Ok(token)
    }
}

pub struct IntArgument;

impl Argument for IntArgument {
    type Output = i32;

    fn parse(token: String) -> Result<Self::Output> {
        token.parse().map_err(|_| Error::InvalidCommand(InvalidCommandReason::InvalidArgument))
    }
}
