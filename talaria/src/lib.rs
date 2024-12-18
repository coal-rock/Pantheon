pub mod client;
pub mod server;

use std::net::SocketAddr;

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
    pub instruction: AgentInstructionBody,
}

impl AgentInstruction {
    pub fn serialize(response: &AgentInstruction) -> Vec<u8> {
        bincode::serialize(response).unwrap()
    }

    pub fn deserialize(response: &Vec<u8>) -> AgentInstruction {
        bincode::deserialize(&response[..]).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgentResponse {
    pub packet_header: PacketHeader,
    pub packet_body: AgentResponseBody,
}

impl AgentResponse {
    pub fn serialize(response: &AgentResponse) -> Vec<u8> {
        bincode::serialize(response).unwrap()
    }

    pub fn deserialize(response: &Vec<u8>) -> AgentResponse {
        bincode::deserialize::<AgentResponse>(&response[..]).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Agent {
    pub nickname: Option<String>,
    pub id: u64,
    pub os: Option<String>,
    pub ip: SocketAddr,
    pub last_response_send: u64,
    pub last_response_recv: u64,
    pub instruction_history: Vec<AgentInstruction>,
    pub response_history: Vec<AgentResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgentInfo {
    pub name: String,
    pub id: u64,
    pub ip: String,
    pub status: bool,
    pub ping: u64,
}
