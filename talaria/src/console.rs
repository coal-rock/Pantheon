use serde::{Deserialize, Serialize};
use strum::EnumProperty;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumProperty;
use strum_macros::IntoStaticStr;
use thiserror::Error;

// refers to agent via name or id, ex:
// connect agent1
// connect 12390122898
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AgentIdentifier {
    Nickname { nickname: String },
    ID { id: u64 },
}

// FIXME: hack to comply with EnumIter trait
impl Default for AgentIdentifier {
    fn default() -> Self {
        AgentIdentifier::ID { id: 0 }
    }
}

impl Into<TargetIdentifier> for AgentIdentifier {
    fn into(self) -> TargetIdentifier {
        TargetIdentifier::Agent { agent: self }
    }
}

// refers to group of agents or single agent
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TargetIdentifier {
    Group { group: String },
    Agent { agent: AgentIdentifier },
}

// FIXME: hack to comply with EnumIter trait
impl Default for TargetIdentifier {
    fn default() -> Self {
        TargetIdentifier::Agent {
            agent: AgentIdentifier::ID { id: 0 },
        }
    }
}

impl ToString for TargetIdentifier {
    fn to_string(&self) -> String {
        match self {
            TargetIdentifier::Group { group } => format!("#{}", group),
            TargetIdentifier::Agent { agent } => match agent {
                AgentIdentifier::Nickname { nickname } => format!("@{}", nickname),
                AgentIdentifier::ID { id } => format!("@{}", id),
            },
        }
    }
}

pub trait CommandHelp {
    fn help() -> String;
}

impl<T: IntoEnumIterator + EnumProperty> CommandHelp for T {
    fn help() -> String {
        let mut output = String::new();
        let mut lines = vec![];

        let mut longest_len = 0;
        for command in T::iter() {
            if command.get_str("command").unwrap() == "_" {
                continue;
            }

            let mut line = String::new();

            let padding = "   ";
            line.push_str(&padding);
            line.push_str(command.get_str("command").unwrap());

            let mut args = vec![];

            // Strum doesn't allow us to define non-primitives as props,
            // so for now we're doing this. Moderately hacky, but fine.
            command.get_str("arg1").map(|x| args.push(x));
            command.get_str("arg2").map(|x| args.push(x));
            command.get_str("arg3").map(|x| args.push(x));
            command.get_str("arg4").map(|x| args.push(x));

            match args.len() > 0 && line.len() > padding.len() {
                true => line.push_str(" "),
                false => {}
            }

            line.push_str(&args.join(" "));
            lines.push(line.clone());

            if line.len() > longest_len {
                longest_len = line.len();
            }
        }

        let target_width = longest_len + 3;
        for (idx, command) in T::iter().enumerate() {
            if command.get_str("command").unwrap() == "_" {
                continue;
            }

            output.push_str(&lines[idx]);
            output.push_str(&" ".repeat(target_width - lines[idx].len()));
            output.push_str("| ");
            output.push_str(command.get_str("description").unwrap());

            if idx != lines.len() - 1 {
                output.push_str("\n");
            }
        }

        output
    }
}

pub trait CommandComplete {
    fn complete() -> Vec<String>;
}

impl<T: IntoEnumIterator + EnumProperty> CommandComplete for T {
    fn complete() -> Vec<String> {
        T::iter()
            .map(|x| x.get_str("command").unwrap().to_string())
            .filter(|x| x != "_" && x != "")
            .collect()
    }
}

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumProperty, EnumIter, IntoStaticStr,
)]
pub enum Command {
    #[strum(props(
        command = "connect",
        arg1 = "<target>",
        description = "Connects to an agent or group"
    ))]
    Connect { target: TargetIdentifier },

    #[strum(props(
        command = "disconnect",
        description = "Disconnects from an agent or group"
    ))]
    Disconnect,

    #[strum(props(
        command = "nickname",
        arg1 = "<set | get | clear>",
        arg2 = "[agent]",
        description = "Modifies nicknames"
    ))]
    Nickname(NicknameCommand),

    #[strum(props(
        command = "group",
        arg1 = "<create | delete | add | remove | clear>",
        arg2 = "<group>",
        arg3 = "[agent..]",
        description = "Modifies groups",
    ))]
    Group(GroupCommand),

    #[strum(props(
        command = "show",
        arg1 = "<agents | groups | server | scripts | stats | [target]>",
        description = "Displays information",
    ))]
    Show(ShowCommand),

    #[strum(props(
        command = "run",
        arg1 = "<script | rhai | shell>",
        arg2 = "[target]",
        arg3 = "<name | content | command>",
        description = "Executes payload on specified target"
    ))]
    Run(RunCommand),

    #[strum(props(
        command = "remove",
        arg1 = "[target..]",
        description = "Kills agent and disconnects from Tartarus"
    ))]
    Remove { target: Vec<TargetIdentifier> },

    #[strum(props(command = "clear", description = "Clears the screen"))]
    Clear,

    #[default]
    #[strum(props(command = "help", description = "Displays help menu"))]
    Help,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumProperty, EnumIter, Default)]
pub enum ShowCommand {
    #[default]
    #[strum(props(command = "agents", description = "Shows information about all agents"))]
    Agents,
    #[strum(props(command = "groups", description = "Shows information about all groups"))]
    Groups,
    #[strum(props(
        command = "server",
        description = "Shows information about hosting server"
    ))]
    Server,
    #[strum(props(command = "stats", description = "Shows statistics"))]
    Stats,
    #[strum(props(command = "scripts", description = "Shows available scripts"))]
    Scripts,
    #[strum(props(
        command = "",
        arg1 = "[target]",
        description = "Shows information about specified target"
    ))]
    Target(Option<TargetIdentifier>),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, EnumProperty, EnumIter)]
pub enum NicknameCommand {
    #[strum(props(
        command = "set",
        arg1 = "[agent]",
        arg2 = "<new nickname>",
        description = "Sets an agents nickname"
    ))]
    Set {
        agent: Option<AgentIdentifier>,
        nickname: String,
    },
    #[strum(props(
        command = "get",
        arg1 = "[agent]",
        description = "Retrieves nickname for a specified agent"
    ))]
    Get { agent: Option<AgentIdentifier> },

    #[strum(props(
        command = "clear",
        arg1 = "[agent]",
        description = "Removes nickname for a specified agent"
    ))]
    Clear { agent: Option<AgentIdentifier> },

    #[default]
    #[strum(props(command = "_"))]
    None,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumProperty, EnumIter)]
pub enum GroupCommand {
    #[strum(props(
        command = "create",
        arg1 = "<group>",
        arg2 = "<agent..>",
        description = "Creates a new group and adds specified agents to it"
    ))]
    Create {
        group_name: String,
        agents: Vec<AgentIdentifier>,
    },
    #[strum(props(
        command = "delete",
        arg1 = "<group>",
        description = "Deletes a specified group. Agents remain unchanged"
    ))]
    Delete { group_name: String },
    #[strum(props(
        command = "add",
        arg1 = "<group>",
        arg2 = "<agent..>",
        description = "Adds specified agent(s) to an existing group"
    ))]
    Add {
        group_name: String,
        agents: Vec<AgentIdentifier>,
    },
    #[strum(props(
        command = "remove",
        arg1 = "<group>",
        arg2 = "<agent..>",
        description = "Removes specified agent(s) from an existing group"
    ))]
    Remove {
        group_name: String,
        agents: Vec<AgentIdentifier>,
    },

    #[strum(props(
        command = "clear",
        arg1 = "<group>",
        description = "Removes all agents from a specified group"
    ))]
    Clear { group_name: String },

    #[default]
    #[strum(props(command = "_"))]
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumProperty, EnumIter)]
pub enum Param {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<Param>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumProperty, Default, EnumIter)]
pub enum RunCommand {
    #[strum(props(
        command = "script",
        arg1 = "[target]",
        arg2 = "<script name>",
        description = "Executes a Rhai script on a specified target"
    ))]
    Script {
        target: Option<TargetIdentifier>,
        script_name: String,
        params: Vec<Param>,
    },

    #[strum(props(
        command = "rhai",
        arg1 = "[target]",
        arg2 = "<rhai source>",
        description = "Executes some Rhai code on a specified target"
    ))]
    Rhai {
        target: Option<TargetIdentifier>,
        scripts_contents: String,
    },

    #[strum(props(
        command = "shell",
        arg1 = "[target]",
        arg2 = "<shell command>",
        description = "Executes some shell command on a specified target"
    ))]
    Shell {
        target: Option<TargetIdentifier>,
        shell_command: String,
    },

    #[default]
    #[strum(props(command = "_", description = "_"))]
    None,
}

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
pub enum CommandError {
    #[error("unknown command: {command_name}")]
    UnknownCommand { command_name: String },
    #[error("invalid agent id")]
    InvalidAgentId,
    #[error("invalid agent nickname")]
    InvalidAgentNickname,
    #[error("invalid script name")]
    InvalidScriptName,
    #[error("group must start with #")]
    GroupMustStartWithPound,
    #[error("agent must start with @")]
    AgentMustStartWithAt,
    #[error("nickname must start with @")]
    NicknameMustStartWith,
    #[error("target must start with @ or #")]
    IdentifierMustStartWith,
    #[error("invalid agent identifier")]
    InvalidAgentIdentifier,
    #[error("expected an argument")]
    ExpectedArgument,
    #[error("expected script name")]
    ExpectedScriptName,
    #[error("expected group")]
    ExpectedGroupIdentifier,
    #[error("expected agent")]
    ExpectedAgentIdentifier,
    #[error("expected target")]
    ExpectedIdentifier,
    #[error("expected nickname")]
    ExpectedAgentNickname,
    #[error("expected command")]
    ExpectedCommand,
    #[error("{}", ._0)]
    ExpectedCommandSpecific(String),
    #[error("unexpected argument: \"{arg}\"")]
    UnexpectedArgument { arg: String },
    #[error("expected {args} args")]
    ExpectedNArgs { args: u64 },
    #[error("expected {args1} or {args2} args")]
    ExpectedAOrBArgs { args1: u64, args2: u64 },
    #[error("unable to parse command")]
    ParsingError,
    #[error("expected rhai script")]
    ExpectedRhai,
    #[error("expected shell script")]
    ExpectedShell,
    #[error("expected script param")]
    ExpectedScriptParam,
    #[error("expected closing bracket")]
    ExpectedClosingBracket,
    #[error("invalid number")]
    InvalidNumber,
}

pub struct Parser {
    source: Vec<String>,
    pos: usize,
    completion: Option<String>,
}

impl Parser {
    pub fn new(source: Vec<String>) -> Parser {
        Parser {
            source,
            pos: 0,
            completion: None,
        }
    }

    pub fn tokenize(source: String) -> Vec<String> {
        let source: Vec<char> = source.chars().collect();

        let mut tokens: Vec<String> = vec![];
        let mut in_quotes = false;
        let mut in_ticks = false;
        let mut escape_next = false;
        let mut current_token: Vec<char> = vec![];

        for char in source.clone() {
            // if last char was "\", escape the next character and ignore the "\"
            if escape_next {
                current_token.push(char);
                escape_next = false;
            }
            // if current char is a backslash, escape the next char
            else if char == '\\' {
                escape_next = true;
            }
            // if we're currently in qoutes, and the current char is a quote,
            // add the token buffer to the list of tokens, if not, add the current char
            // to the token buffer
            else if in_quotes {
                if char == '"' {
                    in_quotes = false;
                    tokens.push(current_token.iter().collect());
                    current_token.clear();
                } else {
                    current_token.push(char);
                }
            } else if in_ticks {
                if char == '`' {
                    in_ticks = false;
                    tokens.push(current_token.iter().collect());
                    current_token.clear();
                } else {
                    current_token.push(char);
                }
            } else {
                // if token is '"', then we start a new token buffer and add the old one to the
                // list of tokens
                if char == '"' && !in_ticks {
                    in_quotes = true;

                    if current_token.len() > 0 {
                        tokens.push(current_token.iter().collect());
                        current_token.clear();
                    }
                } else if char == '`' && !in_quotes {
                    in_ticks = true;

                    if current_token.len() > 0 {
                        tokens.push(current_token.iter().collect());
                        current_token.clear();
                    }
                }
                // break tokens on space
                else if char == ' ' {
                    if current_token.len() > 0 {
                        tokens.push(current_token.iter().collect());
                        current_token.clear();
                    }
                } else if char == '[' || char == ']' {
                    if current_token.len() == 0 {
                        tokens.push(char.to_string());
                    } else {
                        current_token.push(char);
                    }
                } else {
                    current_token.push(char);
                }
            }
        }

        if current_token.len() > 0 {
            tokens.push(current_token.iter().collect());
        }

        tokens
    }

    pub fn consume(&mut self, err: CommandError) -> Result<&str, CommandError> {
        if self.is_at_end() {
            return Err(err);
        }

        self.pos += 1;
        return Ok(&self.source[self.pos - 1]);
    }

    pub fn peek(&self, err: CommandError) -> Result<&str, CommandError> {
        if !self.is_at_end() {
            return Ok(&self.source[self.pos]);
        }

        Err(err)
    }

    pub fn is_at_end(&self) -> bool {
        self.pos == self.source.len()
    }

    pub fn parse_target_ident(&mut self) -> Result<TargetIdentifier, CommandError> {
        let token = self.peek(CommandError::ExpectedIdentifier)?;
        let next_char = token.chars().next().ok_or(CommandError::ParsingError)?;

        // match on first char
        match next_char {
            '#' => {
                return Ok(TargetIdentifier::Group {
                    group: self.parse_group_ident()?,
                })
            }
            '@' => {
                return Ok(TargetIdentifier::Agent {
                    agent: self.parse_agent_ident()?,
                })
            }
            _ => return Err(CommandError::IdentifierMustStartWith),
        };
    }

    /// WARNING: this likely will not work with two idents back to back
    pub fn parse_opt_target_ident(
        &mut self,
        last_arg: bool,
    ) -> Result<Option<TargetIdentifier>, CommandError> {
        match last_arg {
            true =>
            // his is a little bit of a hack,
            // but it's very ergonomic to use-- and i'm not sure how to
            // implement this in a nicer way
            //
            // this implementation breaks down if .peek() starts
            // doing additional error handling (which it shouldn't?)
            {
                match self.peek(CommandError::ParsingError) {
                    Ok(token) => {
                        match token.chars().next().ok_or(CommandError::ExpectedArgument)? {
                            '@' | '#' => Ok(Some(self.parse_target_ident()?)),
                            _ => Err(CommandError::IdentifierMustStartWith),
                        }
                    }
                    Err(_) => Ok(None),
                }
            }
            false => {
                let mut chars = self.peek(CommandError::ExpectedIdentifier)?.chars();
                let predicate = chars.next().ok_or(CommandError::ExpectedIdentifier)?;

                match predicate == '@' || predicate == '#' {
                    true => Ok(Some(self.parse_target_ident()?)),
                    false => Ok(None),
                }
            }
        }
    }

    pub fn parse_target_ident_vec(&mut self) -> Result<Vec<TargetIdentifier>, CommandError> {
        let mut targets = vec![];

        while !self.is_at_end() {
            targets.push(self.parse_target_ident()?);
        }

        targets.dedup();

        Ok(targets)
    }

    pub fn parse_group_ident(&mut self) -> Result<String, CommandError> {
        let token = self.consume(CommandError::ExpectedGroupIdentifier)?;

        if token.starts_with("#") {
            return Ok(token[1..token.len()].to_string());
        }

        return Err(CommandError::GroupMustStartWithPound);
    }

    pub fn parse_agent_ident(&mut self) -> Result<AgentIdentifier, CommandError> {
        let token = self.peek(CommandError::ExpectedAgentIdentifier)?;

        let mut chars = token.chars();
        let predicate = chars.next().ok_or(CommandError::ParsingError)?;
        let first_char = chars.next().ok_or(CommandError::ParsingError)?;

        if predicate != '@' {
            return Err(CommandError::AgentMustStartWithAt);
        }

        match first_char {
            '0'..='9' => Ok(AgentIdentifier::ID {
                id: self.parse_agent_id()?,
            }),
            'a'..='z' | 'A'..='Z' => Ok(AgentIdentifier::Nickname {
                nickname: self.parse_agent_nickname()?,
            }),
            _ => Err(CommandError::InvalidAgentIdentifier),
        }
    }

    pub fn parse_opt_agent_ident(
        &mut self,
        last_arg: bool,
    ) -> Result<Option<AgentIdentifier>, CommandError> {
        match last_arg {
            true =>
            // this is a little bit of a hack,
            // but it's very ergonomic to use-- and i'm not sure how to
            // implement this in a nicer way
            //
            // this implementation breaks down if .peek() starts
            // doing additional error handling (which it shouldn't?)
            {
                match self.peek(CommandError::ParsingError) {
                    Ok(_) => Ok(Some(self.parse_agent_ident()?)),
                    Err(_) => Ok(None),
                }
            }
            false => {
                // let mut chars = self.peek(CommandError::ExpectedAgentIdentifier)?.chars();
                // let predicate = chars.next().ok_or(CommandError::ExpectedAgentIdentifier)?;
                // let first_char = chars.next().ok_or(CommandError::ParsingError)?;
                //
                // if predicate != '@' {
                //     return Ok(None);
                // }
                //
                // match first_char {
                //     'a'..='z' | 'A'..='Z' => Ok(Some(self.parse_agent_ident()?)),
                //     _ => Ok(None),
                // }
                panic!("deprecateed")
            }
        }
    }

    pub fn parse_agent_ident_vec(&mut self) -> Result<Vec<AgentIdentifier>, CommandError> {
        let mut agents = vec![];

        while !self.is_at_end() {
            agents.push(self.parse_agent_ident()?);
        }

        agents.dedup();

        Ok(agents)
    }

    pub fn parse_agent_id(&mut self) -> Result<u64, CommandError> {
        let token = self.consume(CommandError::ExpectedAgentIdentifier)?;

        let mut chars = token.chars().peekable();
        let predicate = chars.next().ok_or(CommandError::ParsingError)?;
        let first_char = chars.peek().ok_or(CommandError::ParsingError)?;

        if predicate != '@' {
            return Err(CommandError::AgentMustStartWithAt);
        }

        match first_char {
            '0'..='9' => {
                let id = chars.collect::<String>().parse::<u64>();

                match id {
                    Ok(id) => Ok(id),
                    Err(_) => Err(CommandError::InvalidAgentId),
                }
            }
            _ => Err(CommandError::InvalidAgentId),
        }
    }

    pub fn parse_agent_nickname(&mut self) -> Result<String, CommandError> {
        let token = self.consume(CommandError::ExpectedAgentNickname)?;

        let mut chars = token.chars().peekable();
        let predicate = chars.next().ok_or(CommandError::ParsingError)?;
        let first_char = chars.peek().ok_or(CommandError::ParsingError)?;

        if predicate != '@' {
            return Err(CommandError::NicknameMustStartWith);
        }

        match first_char {
            'a'..='z' | 'A'..='Z' => Ok(chars.collect()),
            _ => Err(CommandError::InvalidAgentNickname),
        }
    }

    pub fn parse_command<T>(&mut self) -> Result<T, CommandError>
    where
        T: IntoEnumIterator + EnumProperty,
        T::Iterator: Iterator<Item = T>,
    {
        let command_str = self
            .consume(CommandError::ExpectedCommand)
            .map(|x| x.to_string());

        let command_str = match command_str {
            Ok(command_str) => command_str,
            Err(_) => {
                return Err(CommandError::ExpectedCommandSpecific(T::help()));
            }
        };

        // TODO: this doesn't account for instances where two
        // commands start with the same pattern, in such a case,
        // order may be undefined
        for defined_command in T::iter() {
            let defined_command_str = match defined_command.get_str("command") {
                Some(command) => command,
                None => continue,
            };

            if defined_command_str.starts_with(&command_str) {
                self.completion = Some(defined_command_str.to_string());
                return Ok(defined_command);
            }

            self.completion = None;
        }

        Err(CommandError::UnknownCommand {
            command_name: command_str.to_string(),
        })
    }

    pub fn parse(&mut self) -> Result<Command, CommandError> {
        let command = self.parse_command::<Command>()?;

        let command = Ok(match command {
            Command::Clear => Command::Clear,
            Command::Help => Command::Help,
            Command::Disconnect => Command::Disconnect,
            Command::Connect { .. } => Command::Connect {
                target: self.parse_target_ident()?,
            },
            Command::Remove { .. } => Command::Remove {
                target: self.parse_target_ident_vec()?,
            },
            Command::Nickname(_) => Command::Nickname(self.parse_nickname_command()?),
            Command::Group(_) => Command::Group(self.parse_group_command()?),
            Command::Show(_) => Command::Show(self.parse_show_command()?),
            Command::Run(_) => Command::Run(self.parse_run_command()?),
        });

        if !self.is_at_end() {
            return Err(CommandError::UnexpectedArgument {
                arg: self.consume(CommandError::ParsingError)?.to_string(),
            });
        }

        command
    }

    pub fn auto_complete(&mut self) -> Option<String> {
        let _ = self.parse();

        match &self.completion {
            Some(completion) => {
                // FIXME: hack
                // fixes completion on the following case
                // h eHLP
                // (where the completion is capitalized)
                if !completion.starts_with(self.source.last().unwrap()) {
                    return None;
                }

                return Some(
                    completion
                        .to_string()
                        .replacen(self.source.last().unwrap(), "", 1)
                        + " ",
                );
            }
            None => {
                return None;
            }
        };
    }

    pub fn parse_nickname_command(&mut self) -> Result<NicknameCommand, CommandError> {
        let command = self.parse_command::<NicknameCommand>()?;

        Ok(match command {
            NicknameCommand::Set { .. } => {
                let agent_or_nickname = self.parse_agent_ident()?;

                return match self.is_at_end() {
                    true => match agent_or_nickname {
                        AgentIdentifier::Nickname { nickname } => Ok(NicknameCommand::Set {
                            agent: None,
                            nickname,
                        }),
                        AgentIdentifier::ID { .. } => Err(CommandError::ExpectedAgentNickname),
                    },
                    false => Ok(NicknameCommand::Set {
                        agent: Some(agent_or_nickname),
                        nickname: self.parse_agent_nickname()?,
                    }),
                };
            }
            NicknameCommand::Get { .. } => NicknameCommand::Get {
                agent: self.parse_opt_agent_ident(true)?,
            },
            NicknameCommand::Clear { .. } => NicknameCommand::Clear {
                agent: self.parse_opt_agent_ident(true)?,
            },
            NicknameCommand::None => NicknameCommand::None,
        })
    }

    pub fn parse_group_command(&mut self) -> Result<GroupCommand, CommandError> {
        Ok(match self.parse_command::<GroupCommand>()? {
            GroupCommand::Create { .. } => GroupCommand::Create {
                group_name: self.parse_group_ident()?,
                agents: self.parse_agent_ident_vec()?,
            },
            GroupCommand::Delete { .. } => GroupCommand::Delete {
                group_name: self.parse_group_ident()?,
            },
            GroupCommand::Add { .. } => GroupCommand::Add {
                group_name: self.parse_group_ident()?,
                agents: self.parse_agent_ident_vec()?,
            },
            GroupCommand::Remove { .. } => GroupCommand::Remove {
                group_name: self.parse_group_ident()?,
                agents: self.parse_agent_ident_vec()?,
            },
            GroupCommand::Clear { .. } => GroupCommand::Clear {
                group_name: self.parse_group_ident()?,
            },
            GroupCommand::None => GroupCommand::None,
        })
    }

    pub fn parse_show_command(&mut self) -> Result<ShowCommand, CommandError> {
        Ok(match self.parse_command::<ShowCommand>()? {
            ShowCommand::Agents => ShowCommand::Agents,
            ShowCommand::Groups => ShowCommand::Groups,
            ShowCommand::Server => ShowCommand::Server,
            ShowCommand::Scripts => ShowCommand::Scripts,
            ShowCommand::Stats => ShowCommand::Stats,
            ShowCommand::Target(_) => ShowCommand::Target(self.parse_opt_target_ident(true)?),
        })
    }

    pub fn parse_run_command(&mut self) -> Result<RunCommand, CommandError> {
        Ok(match self.parse_command::<RunCommand>()? {
            RunCommand::Script { .. } => RunCommand::Script {
                target: self.parse_opt_target_ident(false)?,
                script_name: self.parse_script_name()?,
                params: self.parse_script_params()?,
            },
            RunCommand::Rhai { .. } => RunCommand::Rhai {
                target: self.parse_opt_target_ident(false)?,
                scripts_contents: self.consume(CommandError::ExpectedRhai)?.to_string(),
            },
            RunCommand::Shell { .. } => RunCommand::Shell {
                target: self.parse_opt_target_ident(false)?,
                shell_command: self.consume(CommandError::ExpectedShell)?.to_string(),
            },
            RunCommand::None => RunCommand::None,
        })
    }

    pub fn parse_script_name(&mut self) -> Result<String, CommandError> {
        let token = self.consume(CommandError::ExpectedScriptName)?;
        let next_char = token.chars().next().ok_or(CommandError::ParsingError)?;

        match next_char {
            'a'..='z' | 'A'..='Z' => Ok(token.to_string()),
            _ => Err(CommandError::InvalidScriptName),
        }
    }

    pub fn parse_script_params(&mut self) -> Result<Vec<Param>, CommandError> {
        // FIXME:
        Ok(vec![])
        // if self.is_at_end() {
        //     return Ok(vec![]);
        // }
        //
        // let mut params = vec![];
        //
        // loop {
        //     let token = self.peek(CommandError::ExpectedScriptParam);
        //     let token = token?;
        //
        //     let next_char = token
        //         .chars()
        //         .next()
        //         .ok_or(CommandError::ExpectedClosingBracket)?;
        //
        //     match next_char {
        //         'a'..='z' | 'A'..='Z' => {
        //             if token == "true" {
        //                 params.push(Param::Bool(true));
        //             }
        //
        //             if token == "false" {
        //                 params.push(Param::Bool(false));
        //             }
        //
        //             params.push(self.parse_script_string()?);
        //         }
        //         '0'..'9' | '.' | '-' => params.push(self.parse_script_number()?),
        //         '[' => {}
        //         ']' => {}
        //         _ => {}
        //     }
        // }
    }

    pub fn parse_script_number(&mut self) -> Result<Param, CommandError> {
        let token = self.consume(CommandError::ExpectedScriptParam)?;

        match token.parse::<f64>() {
            Ok(float) => return Ok(Param::Float(float)),
            Err(_) => {}
        }

        match token.parse::<i64>() {
            Ok(int) => return Ok(Param::Int(int)),
            Err(_) => {}
        }

        return Err(CommandError::InvalidNumber);
    }

    pub fn parse_script_string(&mut self) -> Result<Param, CommandError> {
        let token = self.consume(CommandError::ExpectedScriptParam);

        Ok(Param::String(token?.to_string()))
    }
}

pub struct Console {
    history: Vec<String>,
    current_target: Option<TargetIdentifier>,
}

impl Console {
    pub fn new(current_target: Option<TargetIdentifier>) -> Console {
        Console {
            history: vec![],
            current_target,
        }
    }

    pub fn status_line(&self) -> String {
        match &self.current_target {
            Some(target) => format!("{} > ", target.to_string()),
            None => format!("> "),
        }
    }

    pub fn set_target(&mut self, current_target: Option<TargetIdentifier>) {
        self.current_target = current_target;
    }

    pub fn get_target(&self) -> Option<TargetIdentifier> {
        self.current_target.clone()
    }

    pub fn handle_command(&mut self, source: String) -> Result<Command, CommandError> {
        self.history.push(source.clone());

        let tokens = Parser::tokenize(source.clone());
        let mut parser = Parser::new(tokens);

        parser.parse()
    }

    pub fn auto_complete(&self, source: String) -> Option<String> {
        let tokens = Parser::tokenize(source.clone());
        let mut parser = Parser::new(tokens);

        match parser.auto_complete() {
            Some(completion) => {
                // don't autocomplete unless there is actually text
                if source.ends_with(" ") {
                    return None;
                }

                return Some(completion);
            }
            None => {
                return None;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandContext {
    pub command: Command,
    pub current_target: Option<TargetIdentifier>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NewTarget {
    NoTarget,
    Target { target: TargetIdentifier },
    NoChange,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConsoleResponse {
    pub output: String,
    pub new_target: NewTarget,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConsoleError {
    pub message: String,
}

impl From<&str> for ConsoleError {
    fn from(value: &str) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<String> for ConsoleError {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}
