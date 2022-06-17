use crate::argument::parser::ArgumentParser;
use crate::argument::Argument;
use crate::parsers::tokenize::{tokenize, Token};
use crate::Result;
pub use builder::*;
pub use exec_context::ExecContext;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

mod builder;
mod exec_context;

pub enum NodeType {
    Argument(Argument),
    Literal(String),
}

pub struct Dispatcher<C: Debug> {
    root: Command<C>,
    prefix: String,
    context_factory: Box<dyn Fn() -> C>,
}

#[allow(clippy::type_complexity)]
pub struct Command<C: Debug> {
    children: Vec<Command<C>>,
    node: NodeType,
    exec: Option<Box<dyn Fn(ExecContext<C>) -> Result<()>>>,
}

impl<C: Debug> Command<C> {
    pub fn literal(name: impl Into<String>) -> CommandBuilder<C> {
        CommandBuilder::literal(name)
    }

    pub fn argument(
        name: impl Into<String>,
        parser: impl ArgumentParser,
        required: bool,
    ) -> CommandBuilder<C> {
        CommandBuilder::argument(parser, name, required)
    }

    fn execute(
        &self,
        tokens: VecDeque<String>,
        named_arguments: HashMap<String, String>,
        context: ExecContext<C>,
    ) -> Result<()> {
        Ok(())
    }
}

impl<C: Debug> Dispatcher<C> {
    pub fn builder() -> DispatcherBuilder<C> {
        DispatcherBuilder::new()
    }

    pub fn run_command(&self, command: &str) -> Result<()> {
        // remove leading whitespace and prefix
        let (command, _) = multispace0(command)?;
        let (command, _) = tag(self.prefix.as_str())(command)?;

        let (_, mut tokens) = tokenize(command)?;
        tokens.push(Token::End);

        let mut cmd_tokens = vec![];
        for token in tokens {
            if token != Token::End {
                cmd_tokens.push(token);
            } else if !cmd_tokens.is_empty() {
                self.execute_command(cmd_tokens)?;
                cmd_tokens = vec![];
            }
        }
        Ok(())
    }

    fn execute_command(&self, tokens: Vec<Token>) -> Result<()> {
        println!("{tokens:#?}");
        let (named_arguments, tokens): (Vec<_>, _) = tokens
            .into_iter()
            .partition(|token| matches!(token, &Token::Named(_, _)));
        let tokens = VecDeque::from(unwrap_tokens(tokens));
        let named_args = map_named_arguments(named_arguments);

        self.root.execute(
            tokens,
            named_args,
            ExecContext::new((self.context_factory)()),
        )
    }
}

fn unwrap_tokens(tokens: Vec<Token>) -> Vec<String> {
    let mut output = vec![];
    for token in tokens {
        if let Token::Simple(content) = token {
            output.push(content);
        }
    }
    output
}

fn map_named_arguments(tokens: Vec<Token>) -> HashMap<String, String> {
    let mut output = HashMap::new();
    for token in tokens {
        if let Token::Named(key, value) = token {
            output.insert(key, value);
        }
    }
    output
}

impl<C: Debug> From<CommandBuilder<C>> for Command<C> {
    fn from(builder: CommandBuilder<C>) -> Self {
        builder.build()
    }
}
