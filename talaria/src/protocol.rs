use anyhow::Result;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Encode, Decode, Serialize, Deserialize)]
pub struct Script {
    pub source: String,
    pub description: String,
    pub title: String,
}

impl ToString for Script {
    fn to_string(&self) -> String {
        format!(
            "Title: {}\nDescription: {}\n\n{}",
            self.title, self.description, self.source
        )
    }
}

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

impl ToString for OSType {
    fn to_string(&self) -> String {
        match self {
            OSType::Windows => String::from("Windows"),
            OSType::Linux => String::from("Linux"),
            OSType::Other => String::from("Other"),
        }
    }
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
    Script(Script),
    Rhai(String),
    Command { command: String, args: Vec<String> },
    Kill,
    Ok,
}

impl AgentInstructionBody {
    pub fn variant(&self) -> &str {
        match self {
            AgentInstructionBody::Command {
                command: _,
                args: _,
            } => "Command",
            AgentInstructionBody::Script(_) => "Script",
            AgentInstructionBody::Ok => "Ok",
            AgentInstructionBody::Rhai(_) => "Rhai",
            AgentInstructionBody::Kill => "Kill",
        }
    }

    pub fn inner_value(&self) -> String {
        match self {
            AgentInstructionBody::Command { command, args } => {
                format!("Command: {}\nArgs: {:#?}", command, args)
            }
            AgentInstructionBody::Ok => String::from("None"),
            AgentInstructionBody::Script(script) => script.title.clone(),
            AgentInstructionBody::Rhai(source) => source.clone(),
            AgentInstructionBody::Kill => String::from("None"),
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
