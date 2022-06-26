use crate::argument::parser::ArgumentParser;
use crate::argument::Argument;
use crate::parsers::tokenize::{tokenize, Token};
use crate::{Error, InvalidCommandReason, Result};
pub use builder::*;
pub use exec_context::ExecContext;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use std::collections::HashMap;
use std::fmt::Debug;

mod builder;
mod exec_context;

pub enum NodeType {
    Argument(Argument),
    Literal(String),
}

enum ExecState<O> {
    Working,
    Done(Result<O>),
}

pub struct Dispatcher<C: Debug, O, B> {
    root: Command<C, O>,
    prefix: String,
    context_factory: fn(&B) -> C,
    base_context: B,
}

#[allow(clippy::type_complexity)]
pub struct Command<C: Debug, O> {
    children: Vec<Command<C, O>>,
    node: NodeType,
    exec: Option<fn(&mut ExecContext<C>) -> Result<O>>,
}

impl<C: Debug, O> Command<C, O> {
    pub fn literal(name: impl Into<String>) -> CommandBuilder<C, O> {
        CommandBuilder::literal(name)
    }

    pub fn argument(
        name: impl Into<String>,
        parser: impl ArgumentParser,
        required: bool,
    ) -> CommandBuilder<C, O> {
        CommandBuilder::argument(parser, name, required)
    }

    fn execute(
        &self,
        mut offset: usize,
        tokens: &[String],
        named_arguments: &mut HashMap<String, String>,
        context: &mut ExecContext<C>,
    ) -> ExecState<O> {
        if offset >= tokens.len() {
            return ExecState::Done(if let Some(exec) = &self.exec {
                exec(context)
            } else {
                Err(Error::InvalidCommand(InvalidCommandReason::UnknownCommand))
            });
        }

        for child in &self.children {
            if child.process(&mut offset, tokens, named_arguments, context) {
                match child.execute(offset, tokens, named_arguments, context) {
                    ExecState::Working => continue,
                    ExecState::Done(res) => return ExecState::Done(res),
                }
            }
        }

        ExecState::Working
    }

    fn process(
        &self,
        offset: &mut usize,
        tokens: &[String],
        named_arguments: &mut HashMap<String, String>,
        context: &mut ExecContext<C>,
    ) -> bool {
        match &self.node {
            NodeType::Literal(name) => {
                if let Some(token) = tokens.get(*offset) {
                    if name == token {
                        *offset += 1;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            NodeType::Argument(argument) => {
                if let Some(named) = named_arguments.get(&argument.name) {
                    if argument.matches(named) {
                        context.insert_argument(argument.name.clone(), named.clone());
                        true
                    } else {
                        !argument.is_required()
                    }
                } else if let Some(token) = tokens.get(*offset) {
                    if argument.matches(token) {
                        *offset += 1;
                        context.insert_argument(argument.name.clone(), token.clone());
                        true
                    } else {
                        !argument.is_required()
                    }
                } else {
                    !argument.is_required()
                }
            }
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(self.node, NodeType::Literal(_))
    }

    pub fn is_argument(&self) -> bool {
        matches!(self.node, NodeType::Argument(_))
    }
}

impl<C: Debug, O, B> Dispatcher<C, O, B> {
    pub fn builder() -> DispatcherBuilder<C, O, B> {
        DispatcherBuilder::new()
    }

    pub fn run_command(&self, command: &str) -> Result<Vec<O>> {
        self.command_in_ctx(command, None)
    }

    pub fn run_command_in_context(
        &self,
        command: &str,
        context: Box<dyn Fn(&B) -> C>,
    ) -> Result<Vec<O>> {
        self.command_in_ctx(command, Some(context))
    }

    fn command_in_ctx(
        &self,
        command: &str,
        context: Option<Box<dyn Fn(&B) -> C>>,
    ) -> Result<Vec<O>> {
        // remove leading whitespace and prefix
        let (command, _) = multispace0(command)?;
        let (command, _) = tag(self.prefix.as_str())(command)?;

        let (_, mut tokens) = tokenize(command)?;
        tokens.push(Token::End);

        let mut cmd_tokens = vec![];
        let mut outputs = vec![];
        for token in tokens {
            if token != Token::End {
                cmd_tokens.push(token);
            } else if !cmd_tokens.is_empty() {
                let context = match &context {
                    Some(factory) => factory(&self.base_context),
                    None => (self.context_factory)(&self.base_context),
                };

                match self.execute_command(cmd_tokens, context) {
                    Ok(res) => outputs.push(res),
                    Err(err) => return Err(err),
                }
                cmd_tokens = vec![];
            }
        }
        Ok(outputs)
    }

    fn execute_command(&self, tokens: Vec<Token>, context: C) -> Result<O> {
        let (named_arguments, tokens): (Vec<_>, _) = tokens
            .into_iter()
            .partition(|token| matches!(token, &Token::Named(_, _)));
        let tokens = unwrap_tokens(tokens);
        let mut named_args = map_named_arguments(named_arguments);

        match self.root.execute(
            0,
            tokens.as_slice(),
            &mut named_args,
            &mut ExecContext::new(context),
        ) {
            ExecState::Working => Err(Error::InvalidCommand(InvalidCommandReason::UnknownCommand)),
            ExecState::Done(res) => res,
        }
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

impl<C: Debug, O> From<CommandBuilder<C, O>> for Command<C, O> {
    fn from(builder: CommandBuilder<C, O>) -> Self {
        builder.build()
    }
}
