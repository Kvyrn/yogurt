use crate::{Error, InvalidCommandReason, Result};
use std::fmt::Debug;

pub trait ArgumentParser: Debug + Clone {
    type Output;

    fn parse(&self, token: &str) -> Result<Self::Output>;

    fn validator(&self) -> fn(&str) -> bool;
}

#[derive(Debug, Clone)]
pub struct StringArgument;

impl ArgumentParser for StringArgument {
    type Output = String;

    fn parse(&self, token: &str) -> Result<Self::Output> {
        Ok(token.to_string())
    }

    fn validator(&self) -> fn(&str) -> bool {
        |_| true
    }
}

#[derive(Debug, Clone)]
pub struct IntArgument;

impl ArgumentParser for IntArgument {
    type Output = i32;

    fn parse(&self, token: &str) -> Result<Self::Output> {
        token
            .parse()
            .map_err(|_| Error::InvalidCommand(InvalidCommandReason::InvalidArgument))
    }

    fn validator(&self) -> fn(&str) -> bool {
        |str| str.parse::<i32>().is_ok()
    }
}

#[derive(Debug, Clone)]
pub struct BoundedIntArgument {
    min: i32,
    max: i32,
}

impl ArgumentParser for BoundedIntArgument {
    type Output = i32;

    fn parse(&self, token: &str) -> Result<Self::Output> {
        let int: i32 = token
            .parse()
            .map_err(|_| Error::InvalidCommand(InvalidCommandReason::InvalidArgument))?;
        if int <= self.max && int >= self.min {
            Ok(int)
        } else {
            Err(Error::InvalidCommand(InvalidCommandReason::InvalidArgument))
        }
    }

    fn validator(&self) -> fn(&str) -> bool {
        |str| str.parse::<i32>().is_ok()
    }
}

#[derive(Debug, Clone)]
pub struct ChoiceArgument(Vec<String>);

impl ArgumentParser for ChoiceArgument {
    type Output = String;

    fn parse(&self, token: &str) -> Result<Self::Output> {
        let token = token.to_string();
        if self.0.contains(&token) {
            Ok(token)
        } else {
            Err(Error::InvalidCommand(InvalidCommandReason::InvalidArgument))
        }
    }

    fn validator(&self) -> fn(&str) -> bool {
        |_| true
    }
}
