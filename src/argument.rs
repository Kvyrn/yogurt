use crate::{Error, InvalidCommandReason, Result};

pub trait Argument {
    type Output;

    fn parse(&self, token: String) -> Result<Self::Output>;
}

pub struct StringArgument;

impl Argument for StringArgument {
    type Output = String;

    fn parse(&self, token: String) -> Result<Self::Output> {
        Ok(token)
    }
}

pub struct IntArgument;

impl Argument for IntArgument {
    type Output = i32;

    fn parse(&self, token: String) -> Result<Self::Output> {
        token.parse().map_err(|_| Error::InvalidCommand(InvalidCommandReason::InvalidArgument))
    }
}

pub struct BoundedIntArgument {
    min: i32,
    max: i32
}

impl Argument for BoundedIntArgument {
    type Output = i32;

    fn parse(&self, token: String) -> Result<Self::Output> {
        let int: i32 = token.parse().map_err(|_| Error::InvalidCommand(InvalidCommandReason::InvalidArgument))?;
        if int <= self.max && int >= self.min {
            Ok(int)
        } else {
            Err(Error::InvalidCommand(InvalidCommandReason::InvalidArgument))
        }
    }
}
