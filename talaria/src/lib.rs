pub mod protocol {
    use core::fmt;

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
