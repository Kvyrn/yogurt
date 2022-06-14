use std::collections::{HashMap, VecDeque};

use fnv::FnvHashMap;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;

use crate::parsers::tokenize::{tokenize, Token};
use crate::{Error, InvalidCommandReason, Result};

type CommandExec = fn() -> Result<()>;

pub struct CommandDispatcher {
    pub commands: FnvHashMap<String, Command>,
    pub prefix: String,
}

pub struct Command {
    parameters: (),
    subcommands: FnvHashMap<String, Command>,
    exec: Option<CommandExec>
}

impl Command {
    pub fn execute(
        &self,
        tokens: VecDeque<String>,
        named_arguments: HashMap<String, String>,
    ) -> Result<()> {
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
        Ok(())
    }

    fn execute_command(&self, tokens: Vec<Token>) -> Result<()> {
        println!("{tokens:#?}");
        let (named_arguments, tokens): (Vec<_>, _) = tokens
            .into_iter()
            .partition(|token| matches!(token, &Token::Named(_, _)));
        let mut tokens = VecDeque::from(unwrap_tokens(tokens));
        let named_args = map_named_arguments(named_arguments);

        let cmd = self.commands.get(
            tokens
                .pop_front()
                .ok_or(Error::InvalidCommand(InvalidCommandReason::UnknownCommand))?
                .as_str(),
        );

        if let Some(cmd) = cmd {
            return cmd.execute(tokens, named_args);
        }

        Err(Error::InvalidCommand(InvalidCommandReason::UnknownCommand))
    }

    pub fn builder() -> CommandDispatcherBuilder {
        CommandDispatcherBuilder::new()
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

#[derive(Default)]
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

#[derive(Default)]
pub struct CommandBuilder {
    subcommands: FnvHashMap<String, Command>,
    parameters: (),
    exec: Option<CommandExec>
}

impl CommandBuilder {
    pub fn new() -> Self {
        CommandBuilder {
            subcommands: FnvHashMap::default(),
            parameters: (),
            exec: None
        }
    }

    pub fn subcommand(mut self, name: &str, command: Command) -> Self {
        self.subcommands.insert(name.to_string(), command);
        self
    }
    
    pub fn exec(mut self, exec: CommandExec) -> Self {
        self.exec = Some(exec);
        self
    }

    pub fn build(self) -> Command {
        Command {
            parameters: self.parameters,
            subcommands: self.subcommands,
            exec: self.exec
        }
    }
}
