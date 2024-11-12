pub mod client;
pub mod server;

use bincode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AgentResponseBody {
    CommandResponse {
        command: String,
        command_id: u32,
        status_code: u8,
        result: String,
    },
    Ok {
        packet_id: u32,
    },
    SystemInfo {
        os: String,
        ip: String,
    },
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
    pub response: AgentResponseBody,
}

impl AgentResponse {
    pub fn serialize(response: &AgentResponse) -> Vec<u8> {
        bincode::serialize(response).unwrap()
    }

    pub fn deserialize(response: &Vec<u8>) -> AgentResponse {
        bincode::deserialize::<AgentResponse>(&response[..]).unwrap()
    }
}
