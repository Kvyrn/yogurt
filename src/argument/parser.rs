use crate::{Error, InvalidCommandReason, Result};
use std::fmt::Debug;

pub trait ArgumentParser: Debug + Clone {
    type Output;

    fn parse(&self, token: &str) -> Result<Self::Output>;

    fn validator(&self) -> Box<dyn Fn(String) -> bool>;
}

#[derive(Debug, Clone)]
pub struct StringArgument;

impl ArgumentParser for StringArgument {
    type Output = String;

    fn parse(&self, token: &str) -> Result<Self::Output> {
        Ok(token.to_string())
    }

    fn validator(&self) -> Box<dyn Fn(String) -> bool> {
        Box::new(|_| true)
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

    fn validator(&self) -> Box<dyn Fn(String) -> bool> {
        Box::new(|str| str.parse::<i32>().is_ok())
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

    fn validator(&self) -> Box<dyn Fn(String) -> bool> {
        let min = self.min;
        let max = self.max;
        Box::new(move |str| match str.parse::<i32>() {
            Ok(int) => int <= max && int >= min,
            Err(_) => false,
        })
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

    fn validator(&self) -> Box<dyn Fn(String) -> bool> {
        let options = self.0.clone();
        Box::new(move |token| options.contains(&token))
    }
}
