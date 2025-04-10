pub mod protocol {
    use anyhow::Result;
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    #[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct OS {
        pub os_type: OSType,
        pub os_string: Option<String>,
    }

    impl OS {
        pub fn from(os_type: &str, os_string: Option<String>) -> OS {
            println!("os type: {}", os_type);
            OS {
                os_type: match os_type.to_lowercase().as_str() {
                    "linux" => OSType::Linux,
                    "windows" => OSType::Windows,
                    _ => OSType::Other,
                },
                os_string,
            }
        }
    }

    #[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub enum OSType {
        Windows,
        Linux,
        Other,
    }

    #[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
    pub enum AgentResponseBody {
        CommandResponse {
            command: String,
            status_code: i32,
            stdout: String,
            stderr: String,
        },
        ScriptResponse,
        Ok,
        SystemInfo {},
        Heartbeat,
        Error,
    }

    impl AgentResponseBody {
        pub fn variant(&self) -> &str {
            match self {
                AgentResponseBody::CommandResponse {
                    command: _,
                    status_code: _,
                    stdout: _,
                    stderr: _,
                } => "CommandResponse",
                AgentResponseBody::Ok => "Ok",
                AgentResponseBody::SystemInfo {} => "SystemInfo",
                AgentResponseBody::Heartbeat => "Heartbeat",
                AgentResponseBody::Error => "Error",
                AgentResponseBody::ScriptResponse => "ScriptResponse",
            }
        }

        pub fn inner_value(&self) -> String {
            match self {
                AgentResponseBody::CommandResponse {
                    command,
                    status_code,
                    stdout,
                    stderr,
                } => format!(
                    "Command: {}\nStatus Code: {}\nstdout: {}\nstderr: {}",
                    command, status_code, stdout, stderr
                ),
                AgentResponseBody::Ok => String::from("None"),
                AgentResponseBody::SystemInfo {} => String::from("None"),
                AgentResponseBody::Heartbeat => String::from("None"),
                AgentResponseBody::Error => String::from("None"),
                AgentResponseBody::ScriptResponse => String::from("None"),
            }
        }
    }

    #[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
    pub enum AgentInstructionBody {
        Script { script: String },
        Command { command: String, args: Vec<String> },
        Ok,
    }

    impl AgentInstructionBody {
        pub fn variant(&self) -> &str {
            match self {
                AgentInstructionBody::Command {
                    command: _,
                    args: _,
                } => "Command",
                AgentInstructionBody::Script { script: _ } => "Script",
                AgentInstructionBody::Ok => "Ok",
            }
        }

        pub fn inner_value(&self) -> String {
            match self {
                AgentInstructionBody::Command { command, args } => {
                    format!("Command: {}\nArgs: {:#?}", command, args)
                }
                AgentInstructionBody::Ok => String::from("None"),
                AgentInstructionBody::Script { script } => script.into(),
            }
        }
    }

    // This struct should exclusively contain fields required for minimum viable operation
    // Other data should be locked behind other commands
    #[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
    pub struct ResponseHeader {
        pub ping: Option<u32>,
        pub agent_id: u64,
        pub timestamp: u128,
        pub packet_id: Option<u32>,
        pub polling_interval_ms: u64,
        pub internal_ip: String,
        pub os: OS,
    }

    #[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
    pub struct InstructionHeader {
        pub packet_id: Option<u32>,
        pub timestamp: u128,
    }

    #[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
    pub struct AgentInstruction {
        pub header: InstructionHeader,
        pub body: AgentInstructionBody,
    }

    impl AgentInstruction {
        pub fn serialize(instruction: &AgentInstruction) -> Result<Vec<u8>> {
            let config = bincode::config::standard();
            Ok(bincode::encode_to_vec(instruction, config)?)
        }

        pub fn deserialize(instruction: &Vec<u8>) -> Result<AgentInstruction> {
            let config = bincode::config::standard();
            Ok(bincode::decode_from_slice(instruction, config)?.0)
        }
    }

    #[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
    pub struct AgentResponse {
        pub header: ResponseHeader,
        pub body: AgentResponseBody,
    }

    impl AgentResponse {
        pub fn serialize(response: &AgentResponse) -> Result<Vec<u8>> {
            let config = bincode::config::standard();
            Ok(bincode::encode_to_vec(response, config)?)
        }

        pub fn deserialize(response: &Vec<u8>) -> Result<AgentResponse> {
            let config = bincode::config::standard();
            Ok(bincode::decode_from_slice(response, config)?.0)
        }
    }
}

pub mod api {
    use crate::{helper::current_time, protocol::*};
    use serde::{Deserialize, Serialize};
    use std::{
        collections::{BTreeSet, HashMap, VecDeque},
        net::SocketAddr,
        usize,
    };

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct NetworkHistoryEntry {
        instruction: AgentInstruction,
        response: Option<AgentResponse>,
    }

    /// Layer of abstraction over NetworkHistory.
    ///
    /// Maps timestamp -> ID and then ID -> Entry,
    /// meaning we get O(1) lookups basically for free.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct NetworkHistoryStore {
        by_id: HashMap<u32, NetworkHistoryEntry>,
        by_timestamp: BTreeSet<(u128, u32)>,
        capacity: usize,
    }

    impl NetworkHistoryStore {
        pub fn new(capacity: usize) -> NetworkHistoryStore {
            NetworkHistoryStore {
                by_id: HashMap::new(),
                by_timestamp: BTreeSet::new(),
                capacity,
            }
        }

        /// Inserts or overwrite entry
        pub fn insert(&mut self, entry: NetworkHistoryEntry) {
            let packet_id = match entry.instruction.header.packet_id {
                Some(packet_id) => packet_id,
                None => return,
            };

            let timestamp = entry.instruction.header.timestamp;

            self.by_id.insert(packet_id, entry);
            self.by_timestamp.insert((timestamp, packet_id));

            // trim if we are over capacity
            if self.by_id.len() > self.capacity {
                if let Some(&(oldest_ts, oldest_id)) = self.by_timestamp.iter().next() {
                    self.by_id.remove(&oldest_id);
                    self.by_timestamp.remove(&(oldest_ts, oldest_id));
                }
            }
        }

        /// O(1)
        pub fn get(&self, packet_id: u32) -> Option<&NetworkHistoryEntry> {
            self.by_id.get(&packet_id)
        }

        /// Retrieves all hitherto entries in order of
        /// the instruction timestamp
        pub fn get_all(&self, depth: usize) -> Vec<&NetworkHistoryEntry> {
            self.by_timestamp
                .iter()
                .filter_map(|&(_, packet_id)| self.by_id.get(&packet_id))
                .take(depth)
                .collect()
        }

        /// Creates new entry containing AgentInstruction
        pub fn push_instruction(&mut self, instruction: AgentInstruction) {
            self.insert(NetworkHistoryEntry {
                instruction,
                response: None,
            })
        }

        /// Adds response to existing entry containing an instruction
        pub fn push_response(&mut self, response: AgentResponse) {
            // returns early if response contains no ID
            let entry = match response.header.packet_id {
                Some(packet_id) => self.get(packet_id),
                None => return,
            };

            // returns early if NetworkHistory does not contain matching ID
            // we should never be here(?)
            let entry = match entry {
                Some(entry) => entry,
                None => return,
            };

            self.insert(NetworkHistoryEntry {
                instruction: entry.instruction.clone(),
                response: Some(response),
            });
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Agent {
        pub nickname: Option<String>,
        pub id: u64,
        pub os: OS,
        pub external_ip: SocketAddr,
        // TODO: this maybe shouldn't be a String?
        pub internal_ip: String,
        /// Timestamp of last packet sent from agent (in ms, client time)
        pub last_packet_send: u128,
        /// Timestamp of when last packet from agent was received (in ms, server time)
        pub last_packet_recv: u128,
        /// RTT latency measured in microseconds, will be None for first packet exchange
        pub ping: Option<u32>,
        pub polling_interval_ms: u64,
        pub network_history: NetworkHistoryStore,
        pub instruction_queue: VecDeque<AgentInstructionBody>,
    }

    impl Agent {
        pub fn from_response(response: AgentResponse, external_ip: SocketAddr) -> Agent {
            // TODO: poll max size from config
            let network_history = NetworkHistoryStore::new(1000);

            Agent {
                nickname: None,
                id: response.header.agent_id,
                os: response.header.os,
                external_ip,
                internal_ip: response.header.internal_ip,
                last_packet_send: response.header.timestamp,
                last_packet_recv: current_time(),
                polling_interval_ms: response.header.polling_interval_ms,
                network_history,
                instruction_queue: vec![].into(),
                ping: None,
            }
        }

        pub fn set_nickname(&mut self, nickname: Option<String>) {
            self.nickname = nickname;
        }

        pub fn queue_instruction(&mut self, instruction: &AgentInstructionBody) {
            self.instruction_queue.push_back(instruction.clone());
        }

        pub fn pop_instruction(&mut self) -> Option<AgentInstructionBody> {
            self.instruction_queue.pop_front()
        }

        pub fn is_active(&self) -> bool {
            // TODO: make the timeout count for inactivity be configurable
            // Currently timeout count is set to 3
            (current_time() - self.last_packet_recv) < (self.polling_interval_ms * 3).into()
        }
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct AgentInfo {
        pub name: Option<String>,
        pub os: OS,
        pub id: u64,
        pub external_ip: String,
        pub internal_ip: String,
        pub status: bool,
        /// Ping measured in milliseconds
        pub ping: Option<f32>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct TartarusInfo {
        pub cpu_usage: f32,
        pub memory_total: u64,
        pub memory_used: u64,
        pub storage_total: u64,
        pub storage_used: u64,
        pub cpu_name: String,
        pub core_count: u64,
        pub os: String,
        pub kernel: String,
        pub hostname: String,
        pub uptime: u64,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct TartarusStats {
        pub registered_agents: u64,
        pub active_agents: u64,
        pub packets_sent: u64,
        pub packets_recv: u64,
        pub average_response_latency: f32,
        pub total_traffic: u64,
        pub windows_agents: u64,
        pub linux_agents: u64,
    }
}

pub mod console {
    use serde::{Deserialize, Serialize};
    use strum::EnumProperty;
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    use strum_macros::EnumProperty;
    use thiserror::Error;

    // refers to agent via name or id, ex:
    // connect agent1
    // connect 12390122898
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
    pub enum AgentIdentifier {
        Nickname {
            nickname: String,
        },
        ID {
            id: u64,
        },
        #[default]
        None,
    }

    impl Into<TargetIdentifier> for AgentIdentifier {
        fn into(self) -> TargetIdentifier {
            TargetIdentifier::Agent { agent: self }
        }
    }

    // refers to group of agents or single agent
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
    pub enum TargetIdentifier {
        Group {
            group: String,
        },
        Agent {
            agent: AgentIdentifier,
        },
        #[default]
        None,
    }

    impl ToString for TargetIdentifier {
        fn to_string(&self) -> String {
            match self {
                TargetIdentifier::Group { group } => format!("#{}", group),
                TargetIdentifier::Agent { agent } => match agent {
                    AgentIdentifier::Nickname { nickname } => format!("@{}", nickname),
                    AgentIdentifier::ID { id } => format!("@{}", id),
                    _ => panic!(""),
                },
                _ => panic!(""),
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
                let mut line = String::new();

                line.push_str("   ");
                line.push_str(command.get_str("command").unwrap());

                let mut args = vec![];

                // Strum doesn't allow us to define non-primitives as props,
                // so for now we're doing this. Moderately hacky, but fine.
                command.get_str("arg1").map(|x| args.push(x));
                command.get_str("arg2").map(|x| args.push(x));
                command.get_str("arg3").map(|x| args.push(x));
                command.get_str("arg4").map(|x| args.push(x));

                match args.len() > 0 {
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
            for (idx, command) in Command::iter().enumerate() {
                output.push_str(&lines[idx]);
                output.push_str(&" ".repeat(target_width - lines[idx].len()));
                output.push_str("| ");
                output.push_str(command.get_str("description").unwrap());
                output.push_str("\n");
            }

            output
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumProperty, EnumIter)]
    pub enum Command {
        #[strum(props(
            command = "connect",
            arg1 = "<target>",
            description = "Connects to an agent or group"
        ))]
        Connect { agent: TargetIdentifier },

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
            arg1 = "<agents | groups | server | scripts | [target]>",
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
        #[strum(props(command = "agents"))]
        Agents,
        #[strum(props(command = "groups"))]
        Groups,
        #[strum(props(command = "server"))]
        Server,
        #[strum(props(command = "scripts"))]
        Scripts,
        #[strum(props(command = ""))]
        Target(Option<TargetIdentifier>),
    }

    #[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, EnumProperty, EnumIter)]
    pub enum NicknameCommand {
        #[strum(props(command = "set"))]
        Set {
            agent: Option<AgentIdentifier>,
            nickname: String,
        },
        #[strum(props(command = "get"))]
        Get { agent: Option<AgentIdentifier> },

        #[strum(props(command = "clear"))]
        Clear { agent: Option<AgentIdentifier> },

        #[default]
        None,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumProperty, EnumIter)]
    pub enum GroupCommand {
        #[strum(props(command = "create"))]
        Create {
            group_name: String,
            agents: Vec<AgentIdentifier>,
        },
        #[strum(props(command = "delete"))]
        Delete { group_name: String },
        #[strum(props(command = "add"))]
        Add {
            group_name: String,
            agents: Vec<AgentIdentifier>,
        },
        #[strum(props(command = "remove"))]
        Remove {
            group_name: String,
            agents: Vec<AgentIdentifier>,
        },

        #[strum(props(command = "clear"))]
        Clear { group_name: String },

        #[default]
        None,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumProperty, Default, EnumIter)]
    pub enum RunCommand {
        #[strum(props(command = "script"))]
        Script {
            target: Option<TargetIdentifier>,
            script_name: String,
        },

        #[strum(props(command = "rhai"))]
        Rhai {
            target: Option<TargetIdentifier>,
            scripts_contents: String,
        },

        #[strum(props(command = "shell"))]
        Shell {
            target: Option<TargetIdentifier>,
            shell_command: String,
        },

        #[default]
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
        #[error("expected one of the following commands:\n   {}", ._0.join("\n   "))]
        ExpectedCommandSpecific(Vec<String>),
        #[error("unexpected argument: \"{arg}\"")]
        UnexpectedArgument { arg: String },
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

        pub fn consume(&mut self, err: CommandError) -> Result<&str, CommandError> {
            if !self.is_at_end() {
                self.pos += 1;
                return Ok(&self.source[self.pos - 1]);
            }

            Err(err)
        }

        pub fn consume_to_end(&mut self) -> String {
            self.source[self.pos..].join(" ")
        }

        pub fn peek(&mut self, err: CommandError) -> Result<&str, CommandError> {
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
                // his is a little bit of a hack,
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
                    let mut chars = self.peek(CommandError::ExpectedAgentIdentifier)?.chars();
                    let predicate = chars.next().ok_or(CommandError::ExpectedAgentIdentifier)?;

                    match predicate == '@' {
                        true => Ok(Some(self.parse_agent_ident()?)),
                        false => Ok(None),
                    }
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
                return Err(CommandError::AgentMustStartWithAt);
            }

            match first_char {
                'a'..='z' | 'A'..='Z' => Ok(chars.collect()),
                _ => Err(CommandError::InvalidAgentNickname),
            }
        }

        pub fn parse_command<T: IntoEnumIterator>(&mut self) -> Result<T, CommandError>
        where
            T: IntoEnumIterator + EnumProperty,
            T::Iterator: Iterator<Item = T>,
        {
            let command_str = match self.consume(CommandError::ExpectedCommand) {
                Ok(command_str) => command_str,
                Err(_) => {
                    return Err(CommandError::ExpectedCommandSpecific(
                        T::iter()
                            .map(|x| x.get_str("command"))
                            .filter_map(|x| x)
                            .map(|x| x.to_string())
                            .collect(),
                    ))
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

                if defined_command_str.starts_with(command_str) {
                    return Ok(defined_command);
                }
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
                    agent: self.parse_target_ident()?,
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
                    arg: self.consume_to_end(),
                });
            }

            println!("{:#?}", command);
            command
        }

        pub fn parse_nickname_command(&mut self) -> Result<NicknameCommand, CommandError> {
            Ok(match self.parse_command::<NicknameCommand>()? {
                NicknameCommand::Set { .. } => NicknameCommand::Set {
                    agent: self.parse_opt_agent_ident(false)?,
                    nickname: self.parse_agent_nickname()?,
                },
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
                ShowCommand::Target(_) => ShowCommand::Target(self.parse_opt_target_ident(true)?),
            })
        }

        pub fn parse_run_command(&mut self) -> Result<RunCommand, CommandError> {
            Ok(match self.parse_command::<RunCommand>()? {
                RunCommand::Script { .. } => RunCommand::Script {
                    target: self.parse_opt_target_ident(false)?,
                    script_name: self.parse_script_name()?,
                },
                RunCommand::Rhai { .. } => RunCommand::Rhai {
                    target: self.parse_opt_target_ident(false)?,
                    scripts_contents: self.parse_agent_nickname()?,
                },
                RunCommand::Shell { .. } => RunCommand::Shell {
                    target: self.parse_opt_target_ident(false)?,
                    shell_command: self.parse_agent_nickname()?,
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

            let tokens = Parser::tokenize(source);

            let mut parser = Parser::new(tokens);

            parser.parse()
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

    use std::time::SystemTime;
    pub fn current_time() -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    pub fn current_time_micro() -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }
}
