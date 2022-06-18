use crate::argument::parser::ArgumentParser;
use crate::{Error, InvalidCommandReason, Result};
use fnv::FnvHashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ExecContext<C: Debug> {
    arguments: FnvHashMap<String, String>,
    context: C,
}

impl<C: Debug> ExecContext<C> {
    pub fn new(context: C) -> Self {
        Self {
            arguments: FnvHashMap::default(),
            context,
        }
    }

    pub fn get<A>(
        &self,
        name: impl Into<String>,
        parser: impl ArgumentParser<Output = A>,
    ) -> Result<A> {
        if let Some(token) = self.arguments.get(&name.into()) {
            parser.parse(token)
        } else {
            Err(Error::InvalidCommand(InvalidCommandReason::MissingArgument))
        }
    }

    pub fn context(&self) -> &C {
        &self.context
    }

    pub fn insert_argument(&mut self, name: String, value: String) {
        self.arguments.insert(name, value);
    }
}
