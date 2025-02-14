pub mod protocol {
    use anyhow::Result;
    use bincode;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum AgentResponseBody {
        CommandResponse {
            command: String,
            command_id: u32,
            status_code: i32,
            stdout: String,
            stderr: String,
        },
        Ok {
            packet_id: u32,
        },
        SystemInfo {},
        Heartbeat,
        Error,
    }

    impl AgentResponseBody {
        pub fn variant(&self) -> &str {
            match self {
                AgentResponseBody::CommandResponse {
                    command: _,
                    command_id: _,
                    status_code: _,
                    stdout: _,
                    stderr: _,
                } => "CommandResponse",
                AgentResponseBody::Ok { packet_id: _ } => "Ok",
                AgentResponseBody::SystemInfo {} => "SystemInfo",
                AgentResponseBody::Heartbeat => "Heartbeat",
                AgentResponseBody::Error => "Error",
            }
        }

        pub fn inner_value(&self) -> String {
            match self {
                AgentResponseBody::CommandResponse {
                    command,
                    command_id,
                    status_code,
                    stdout,
                    stderr,
                } => format!(
                    "Command: {}\nCommand ID: {}\nStatus Code: {}\nstdout: {}\nstderr: {}",
                    command, command_id, status_code, stdout, stderr
                ),
                AgentResponseBody::Ok { packet_id } => format!("Packet ID: {}", packet_id),
                AgentResponseBody::SystemInfo {} => String::from("None"),
                AgentResponseBody::Heartbeat => String::from("None"),
                AgentResponseBody::Error => String::from("None"),
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum AgentInstructionBody {
        Command {
            command: String,
            command_id: u32,
            args: Vec<String>,
        },
        RequestHeartbeat,
        Ok,
    }

    impl AgentInstructionBody {
        pub fn variant(&self) -> &str {
            match self {
                AgentInstructionBody::Command {
                    command: _,
                    command_id: _,
                    args: _,
                } => "Command",
                AgentInstructionBody::RequestHeartbeat => "RequestHeartbeat",
                AgentInstructionBody::Ok => "Ok",
            }
        }

        pub fn inner_value(&self) -> String {
            match self {
                AgentInstructionBody::Command {
                    command,
                    command_id,
                    args,
                } => format!(
                    "Command: {}\nCommand ID: {}\nArgs: {:#?}",
                    command, command_id, args
                ),
                AgentInstructionBody::RequestHeartbeat => String::from("None"),
                AgentInstructionBody::Ok => String::from("None"),
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct PacketHeader {
        pub agent_id: u64,
        pub timestamp: u64,
        pub packet_id: u32,
        pub os: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AgentInstruction {
        pub packet_header: PacketHeader,
        pub packet_body: AgentInstructionBody,
    }

    impl AgentInstruction {
        pub fn serialize(response: &AgentInstruction) -> Result<Vec<u8>> {
            Ok(bincode::serialize(response)?)
        }

        pub fn deserialize(response: &Vec<u8>) -> Result<AgentInstruction> {
            Ok(bincode::deserialize(&response[..])?)
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AgentResponse {
        pub packet_header: PacketHeader,
        pub packet_body: AgentResponseBody,
    }

    impl AgentResponse {
        pub fn serialize(response: &AgentResponse) -> Result<Vec<u8>> {
            Ok(bincode::serialize(response)?)
        }

        pub fn deserialize(response: &Vec<u8>) -> Result<AgentResponse> {
            Ok(bincode::deserialize::<AgentResponse>(&response[..])?)
        }
    }
}

pub mod api {
    use crate::protocol::*;
    use serde::{Deserialize, Serialize};
    use std::net::SocketAddr;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum NetworkHistoryEntry {
        AgentInstruction { instruction: AgentInstruction },
        AgentResponse { response: AgentResponse },
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Agent {
        pub nickname: Option<String>,
        pub id: u64,
        pub os: Option<String>,
        pub ip: SocketAddr,
        pub last_packet_send: u64,
        pub last_packet_recv: u64,
        pub network_history: Vec<NetworkHistoryEntry>,
        pub queue: Vec<AgentInstructionBody>,
    }

    impl Agent {
        // appends a response to the network history, used for logging
        pub fn push_response(&mut self, response: &AgentResponse) {
            self.network_history
                .push(NetworkHistoryEntry::AgentResponse {
                    response: response.clone(),
                })
        }

        // appends an instruction to the network history, used for logging
        pub fn push_instruction(&mut self, instruction: &AgentInstruction) {
            self.network_history
                .push(NetworkHistoryEntry::AgentInstruction {
                    instruction: instruction.clone(),
                })
        }

        pub fn get_response_history(&self) -> Vec<AgentResponse> {
            self.network_history
                .iter()
                .filter_map(|x| match x {
                    NetworkHistoryEntry::AgentInstruction { instruction: _ } => None,
                    NetworkHistoryEntry::AgentResponse { response } => Some(response.clone()),
                })
                .collect()
        }

        pub fn get_instruction_history(&self) -> Vec<AgentInstruction> {
            self.network_history
                .iter()
                .filter_map(|x| match x {
                    NetworkHistoryEntry::AgentInstruction { instruction } => {
                        Some(instruction.clone())
                    }
                    NetworkHistoryEntry::AgentResponse { response: _ } => None,
                })
                .collect()
        }

        pub fn set_nickname(&mut self, nickname: Option<String>) {
            self.nickname = nickname;
        }

        pub fn queue_instruction(&mut self, instruction: &AgentInstructionBody) {
            self.queue.push(instruction.clone());
        }

        pub fn pop_instruction(&mut self) -> Option<AgentInstructionBody> {
            self.queue.pop()
        }
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct AgentInfo {
        pub name: String,
        pub id: u64,
        pub ip: String,
        pub status: bool,
        pub ping: u64,
    }
}

pub mod console {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    // refers to agent via name or id, ex:
    // connect agent1
    // connect 12390122898
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum AgentIdentifier {
        Nickname { nickname: String },
        ID { id: u64 },
    }

    // refers to group of agents or single agent
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum TargetIdentifier {
        Group { group: String },
        Agent { agent: AgentIdentifier },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum Command {
        Connect {
            agent: TargetIdentifier,
        },
        Disconnect,
        CreateGroup {
            group_name: String,
            agents: Vec<AgentIdentifier>,
        },
        DeleteGroup {
            group_name: String,
        },
        AddAgentsToGroup {
            group_name: String,
            agents: Vec<AgentIdentifier>,
        },
        RemoveAgentsFromGroup {
            group_name: String,
            agents: Vec<AgentIdentifier>,
        },
        Exec {
            agents: Option<TargetIdentifier>,
            command: String,
        },
        ListAgents,
        ListGroups,
        Ping {
            agents: Option<TargetIdentifier>,
        },
        Status {
            agents: Option<TargetIdentifier>,
        },
        Nickname {
            agent: Option<AgentIdentifier>,
            new_name: String,
        },
        Clear,
        Help,
    }

    #[derive(Error, Clone, Debug, Serialize, Deserialize)]
    pub enum CommandError {
        #[error("unknown command: {command_name}")]
        UnknownCommand { command_name: String },
        #[error("invalid agent id")]
        InvalidAgentId,
        #[error("invalid agent nickname")]
        InvalidAgentNickname,
        #[error("group must start with #")]
        GroupMustStartWithPound,
        #[error("invalid agent identifier")]
        InvalidAgentIdentifier,
        #[error("expected an argument")]
        ExpectedArgument,
        #[error("expected {args} args")]
        ExpectedNArgs { args: u64 },
        #[error("expected {args1} or {args2} args")]
        ExpectedAOrBArgs { args1: u64, args2: u64 },
        #[error("unable to parse command")]
        ParsingError,
    }

    pub enum Token {
        CommandName { command_name: String },
        AgentID { id: u64 },
        AgentNickname { nickname: String },
        GroupIdentifier { identifier: String },
    }

    pub struct Parser {
        source: Vec<String>,
        pos: usize,
    }

    impl Parser {
        pub fn new(source: Vec<String>) -> Parser {
            Parser { source, pos: 0 }
        }

        pub fn tokenize(source: String) -> Vec<String> {
            let source: Vec<char> = source.chars().collect();

            let mut tokens: Vec<String> = vec![];
            let mut in_quotes = false;
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
                } else {
                    // if token is '"', then we start a new token buffer and add the old one to the
                    // list of tokens
                    if char == '"' {
                        in_quotes = true;

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

        pub fn consume(&mut self) -> Result<&str, CommandError> {
            if !self.is_at_end() {
                self.pos += 1;
                return Ok(&self.source[self.pos - 1]);
            }

            Err(CommandError::ExpectedArgument)
        }

        pub fn peek(&mut self) -> Result<&str, CommandError> {
            if !self.is_at_end() {
                return Ok(&self.source[self.pos]);
            }

            Err(CommandError::ExpectedArgument)
        }

        pub fn is_at_end(&self) -> bool {
            self.pos == self.source.len()
        }

        pub fn parse_target_ident(&mut self) -> Result<TargetIdentifier, CommandError> {
            let token = self.peek()?;
            let next_char = token.chars().next().ok_or(CommandError::ParsingError)?;

            // match on first char
            match next_char {
                '#' => {
                    return Ok(TargetIdentifier::Group {
                        group: self.parse_group_ident()?,
                    })
                }
                _ => {
                    return Ok(TargetIdentifier::Agent {
                        agent: self.parse_agent_ident()?,
                    })
                }
            };
        }

        pub fn parse_group_ident(&mut self) -> Result<String, CommandError> {
            let token = self.consume()?;

            if token.starts_with("#") {
                return Ok(token[1..token.len()].to_string());
            }

            return Err(CommandError::GroupMustStartWithPound);
        }

        pub fn parse_agent_ident(&mut self) -> Result<AgentIdentifier, CommandError> {
            let token = self.peek()?;
            let next_char = token.chars().next().ok_or(CommandError::ParsingError)?;

            match next_char {
                '0'..='9' => Ok(AgentIdentifier::ID {
                    id: self.parse_agent_id()?,
                }),
                'a'..='z' | 'A'..='Z' => Ok(AgentIdentifier::Nickname {
                    nickname: self.parse_agent_nickname()?,
                }),
                _ => Err(CommandError::InvalidAgentIdentifier),
            }
        }

        pub fn parse_agent_id(&mut self) -> Result<u64, CommandError> {
            let token = self.consume()?;
            let next_char = token.chars().next().ok_or(CommandError::ParsingError)?;

            match next_char {
                '0'..='9' => {
                    let id = token.parse::<u64>();

                    match id {
                        Ok(id) => Ok(id),
                        Err(_) => Err(CommandError::InvalidAgentId),
                    }
                }
                _ => Err(CommandError::InvalidAgentId),
            }
        }

        pub fn parse_agent_nickname(&mut self) -> Result<String, CommandError> {
            let token = self.consume()?;
            let next_char = token.chars().next().ok_or(CommandError::ParsingError)?;

            match next_char {
                'a'..='z' | 'A'..='Z' => Ok(token.to_string()),
                _ => Err(CommandError::InvalidAgentNickname),
            }
        }

        pub fn parse(&mut self) -> Result<Command, CommandError> {
            let command = self.consume()?;

            match command {
                "connect" => {
                    let target_ident = self.parse_target_ident()?;

                    if self.is_at_end() {
                        Ok(Command::Connect {
                            agent: target_ident,
                        })
                    } else {
                        Err(CommandError::ExpectedNArgs { args: 1 })
                    }
                }
                "disconnect" => {
                    if self.is_at_end() {
                        Ok(Command::Disconnect)
                    } else {
                        Err(CommandError::ExpectedNArgs { args: 0 })
                    }
                }
                "create_group" => {
                    let group_name = self.parse_group_ident()?;

                    let mut agents: Vec<AgentIdentifier> = vec![];

                    while !self.is_at_end() {
                        agents.push(self.parse_agent_ident()?);
                    }

                    Ok(Command::CreateGroup { group_name, agents })
                }
                "delete_group" => {
                    let group_name = self.parse_group_ident()?;

                    match self.is_at_end() {
                        false => Err(CommandError::ExpectedNArgs { args: 1 }),
                        true => Ok(Command::DeleteGroup { group_name }),
                    }
                }
                "add_to_group" => {
                    let group_name = self.parse_group_ident()?;
                    let mut agents: Vec<AgentIdentifier> = vec![];

                    while !self.is_at_end() {
                        agents.push(self.parse_agent_ident()?);
                    }

                    Ok(Command::AddAgentsToGroup { group_name, agents })
                }
                "remove_from_group" => {
                    let group_name = self.parse_group_ident()?;
                    let mut agents: Vec<AgentIdentifier> = vec![];

                    while !self.is_at_end() {
                        agents.push(self.parse_agent_ident()?);
                    }

                    Ok(Command::RemoveAgentsFromGroup { group_name, agents })
                }
                "list_groups" => {
                    if self.is_at_end() {
                        Ok(Command::ListGroups)
                    } else {
                        Err(CommandError::ExpectedNArgs { args: 0 })
                    }
                }
                "exec" => match self.source.len() {
                    2 => Ok(Command::Exec {
                        agents: None,
                        command: self.consume()?.to_string(),
                    }),
                    3 => Ok(Command::Exec {
                        agents: Some(self.parse_target_ident()?),
                        command: self.consume()?.to_string(),
                    }),
                    _ => Err(CommandError::ExpectedAOrBArgs { args1: 1, args2: 2 }),
                },
                "list" => match self.is_at_end() {
                    true => Ok(Command::ListAgents),
                    false => Err(CommandError::ExpectedNArgs { args: 0 }),
                },
                "clear" => match self.is_at_end() {
                    true => Ok(Command::Clear),
                    false => Err(CommandError::ExpectedNArgs { args: 0 }),
                },
                "ping" => match self.source.len() {
                    1 => Ok(Command::Ping { agents: None }),
                    2 => Ok(Command::Ping {
                        agents: Some(self.parse_target_ident()?),
                    }),
                    _ => Err(CommandError::ExpectedAOrBArgs { args1: 0, args2: 1 }),
                },
                "status" => match self.source.len() {
                    1 => Ok(Command::Status { agents: None }),
                    2 => Ok(Command::Status {
                        agents: Some(self.parse_target_ident()?),
                    }),
                    _ => Err(CommandError::ExpectedAOrBArgs { args1: 0, args2: 1 }),
                },
                "nickname" => match self.source.len() {
                    2 => Ok(Command::Nickname {
                        agent: None,
                        new_name: self.consume()?.to_string(),
                    }),
                    3 => Ok(Command::Nickname {
                        agent: Some(self.parse_agent_ident()?),
                        new_name: self.consume()?.to_string(),
                    }),
                    _ => Err(CommandError::ExpectedAOrBArgs { args1: 1, args2: 2 }),
                },
                "help" => {
                    if self.is_at_end() {
                        Ok(Command::Help)
                    } else {
                        Err(CommandError::ExpectedNArgs { args: 0 })
                    }
                }
                _ => Err(CommandError::UnknownCommand {
                    command_name: command.to_string(),
                }),
            }
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
                Some(target) => match target {
                    TargetIdentifier::Group { group } => format!("#{} > ", group),
                    TargetIdentifier::Agent { agent } => match agent {
                        AgentIdentifier::Nickname { nickname } => format!("@{} > ", nickname),
                        AgentIdentifier::ID { id } => format!("@{} > ", id),
                    },
                },
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

            let tokens = Parser::tokenize(source);

            let mut parser = Parser::new(tokens);

            parser.parse()
        }

        pub fn handle_command_response(
            &mut self,
            command_context: CommandContext,
            command_response: ConsoleResponse,
        ) -> String {
            String::new()
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
        pub success: bool,
        pub output: String,
        pub new_target: NewTarget,
    }
}

pub mod helper {
    /// Wrapper around println!() macro that
    /// only runs if the binary was compiled in
    /// debug mode
    #[macro_export]
    macro_rules! devlog{
        ($($rest:expr),+) => {
            {
                #[cfg(debug_assertions)]
                println!($($rest),+);
            }
        };
    }

    pub use devlog;
}
