use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::{alphanumeric1, multispace0};
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::parsers::escaped_string::parse_string;
use crate::Result;

pub fn tokenize(data: &str) -> Result<(&str, Vec<Token>)> {
    let mut output = Vec::<Token>::new();

    let mut data = data;

    while let Ok((remainder, token)) = read_token(data) {
        data = remainder;
        output.push(token);
    }
    Ok((data, output))
}

fn read_token(data: &str) -> Result<(&str, Token)> {
    // remove leading whitespace
    let (data, _) = multispace0(data)?;
    // check command end (;)
    let result: IResult<&str, &str, nom::error::Error<&str>> = tag(";")(data);
    if let Ok((remainder, _)) = result {
        return Ok((remainder, Token::End));
    }

    let (remainder, token) = alt((
        map(
            separated_pair(
                map(alphanumeric1, |s: &str| s.to_string()),
                tag("="),
                alt((parse_string, map(alphanumeric1, |s: &str| s.to_string()))),
            ),
            |(key, value)| Token::Named(key, value),
        ),
        map(parse_string, Token::Simple),
        map(
            take_till1(|c: char| c.is_whitespace() || c.eq(&';')),
            |s: &str| Token::Simple(s.to_string()),
        ),
    ))(data)?;

    Ok((remainder, token))
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    /// Representing a command, subcommand or positional argument
    Simple(String),
    /// Representing an named argument
    Named(String, String),
    /// Representing end of command
    End,
}
