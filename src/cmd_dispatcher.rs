use crate::parsers::tokenize::{tokenize, Token};
use crate::{Error, Result};
use fnv::FnvHashMap;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;

#[derive(Debug, Clone)]
pub struct CommandDispatcher {
    pub commands: FnvHashMap<String, Command>,
    pub prefix: String,
}

#[derive(Debug, Clone)]
pub struct Command {
    parameters: (),
    subcommands: FnvHashMap<String, Command>,
}

impl Command {
    pub fn execute(&self, payload: &str) -> Result<()> {
        todo!()
    }

    pub fn builder() -> CommandBuilder {
        CommandBuilder::new()
    }
}

impl CommandDispatcher {
    pub fn run_command(&self, command: &str) -> Result<()> {
        // remove leading whitespace and prefix
        let (command, _) = multispace0(command)?;
        let (command, _) = tag(self.prefix.as_str())(command)?;

        let (_, mut tokens) = tokenize(command)?;
        if tokens.last() != Some(&Token::End) {
            tokens.push(Token::End)
        }

        let mut cmd_tokens = vec![];
        for token in tokens {
            if token != Token::End {
                cmd_tokens.push(token);
            } else {
                self.execute_command(cmd_tokens)?;
                cmd_tokens = vec![];
            }
        }
        //Ok(())
        Err(Error::ExecutionFailed)
    }

    fn execute_command(&self, tokens: Vec<Token>) -> Result<()> {
        println!("{tokens:#?}");
        Ok(())
    }

    pub fn builder() -> CommandDispatcherBuilder {
        CommandDispatcherBuilder::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommandDispatcherBuilder {
    prefix: String,
    commands: FnvHashMap<String, Command>,
}

impl CommandDispatcherBuilder {
    pub fn new() -> Self {
        CommandDispatcherBuilder {
            prefix: String::new(),
            commands: FnvHashMap::default(),
        }
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    pub fn command(mut self, name: &str, command: Command) -> Self {
        self.commands.insert(name.to_string(), command);
        self
    }

    pub fn build(self) -> CommandDispatcher {
        CommandDispatcher {
            commands: self.commands,
            prefix: self.prefix,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommandBuilder {
    subcommands: FnvHashMap<String, Command>,
    parameters: (),
}

impl CommandBuilder {
    pub fn new() -> Self {
        CommandBuilder {
            subcommands: FnvHashMap::default(),
            parameters: (),
        }
    }

    pub fn subcommand(mut self, name: &str, command: Command) -> Self {
        self.subcommands.insert(name.to_string(), command);
        self
    }

    pub fn build(self) -> Command {
        Command {
            parameters: self.parameters,
            subcommands: self.subcommands,
        }
    }
}
