use super::{ExecContext, NodeType};
use crate::argument::parser::ArgumentParser;
use crate::argument::Argument;
use crate::{Command, Dispatcher, Error, Result};
use std::fmt::Debug;

#[allow(clippy::type_complexity)]
pub struct CommandBuilder<C: Debug, O> {
    children: Vec<Command<C, O>>,
    node: NodeType,
    exec: Option<fn(&mut ExecContext<C>) -> Result<O>>,
}

impl<C: Debug, O> CommandBuilder<C, O> {
    pub fn literal(name: impl Into<String>) -> Self {
        Self {
            children: vec![],
            node: NodeType::Literal(name.into()),
            exec: None,
        }
    }

    pub fn argument(parser: impl ArgumentParser, name: impl Into<String>, required: bool) -> Self {
        Self::argument_validator(parser.validator(), name, required)
    }

    pub fn argument_validator(
        validator: fn(&str) -> bool,
        name: impl Into<String>,
        required: bool,
    ) -> Self {
        Self {
            children: vec![],
            exec: None,
            node: NodeType::Argument(Argument::new(validator, name.into(), required)),
        }
    }

    pub fn exec(mut self, exec: fn(&mut ExecContext<C>) -> Result<O>) -> Self {
        self.exec = Some(exec);
        self
    }

    pub fn child(mut self, child: impl Into<Command<C, O>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn build(self) -> Command<C, O> {
        let (mut literals, arguments): (Vec<_>, Vec<_>) =
            self.children.into_iter().partition(|c| c.is_literal());
        literals.extend(arguments);
        Command {
            children: literals,
            node: self.node,
            exec: self.exec,
        }
    }
}

pub struct DispatcherBuilder<C: Debug, O, B> {
    root: CommandBuilder<C, O>,
    prefix: Option<String>,
    context_factory: Option<fn(&B) -> C>,
    base_context: Option<B>,
}

impl<C: Debug, O, B> DispatcherBuilder<C, O, B> {
    pub fn new() -> Self {
        Self {
            root: CommandBuilder::literal(""),
            prefix: None,
            context_factory: None,
            base_context: None,
        }
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn context_factory(mut self, factory: fn(&B) -> C) -> Self {
        self.context_factory = Some(factory);
        self
    }

    pub fn base_context(mut self, context: B) -> Self {
        self.base_context = Some(context);
        self
    }

    pub fn child(mut self, child: impl Into<Command<C, O>>) -> Self {
        self.root.children.push(child.into());
        self
    }

    pub fn build(self) -> Result<Dispatcher<C, O, B>> {
        Ok(Dispatcher {
            root: self.root.build(),
            prefix: self.prefix.unwrap_or_default(),
            context_factory: self.context_factory.ok_or(Error::IncompleteBuilder)?,
            base_context: self.base_context.ok_or(Error::IncompleteBuilder)?,
        })
    }
}

impl<C: Debug, O, B> Default for DispatcherBuilder<C, O, B> {
    fn default() -> Self {
        Self {
            root: CommandBuilder::literal(""),
            prefix: None,
            context_factory: None,
            base_context: None,
        }
    }
}
