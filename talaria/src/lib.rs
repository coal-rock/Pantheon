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

pub mod console {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use tokio::sync::RwLock;
    use rustyline::Editor;
    use rustyline::history::{FileHistory, History};
    use rocket::yansi::Paint;  // Import yansi for colored output
  // Import the SharedState type from the appropriate module
    use std::time::SystemTime;  // Import SystemTime for timestamp handling
    use crate::protocol::*;
    use crate::api::*;  // Import protocol definitions from the talaria crate
    use crate::console::State as TalariaConsoleState;
    use std::sync::Arc;

    // Reusable structures for agent communication
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct PacketHeader {
        pub agent_id: u64,
        pub timestamp: u64,
        pub packet_id: u32,
        pub os: Option<String>,
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
    pub struct AgentInstruction {
        pub packet_header: PacketHeader,
        pub packet_body: AgentInstructionBody,
    }

    #[derive(Clone, Debug)]
    pub struct Agent {
        pub id: u64,
        pub os: Option<String>,
        pub ip: String,
        pub last_packet_recv: u64,
        pub response_history: Vec<String>,
        pub instructions: Vec<AgentInstruction>,
    }

    impl Agent {
        pub fn new(id: u64, ip: String) -> Self {
            Self {
                id,
                os: None,
                ip,
                last_packet_recv: current_time(),
                response_history: Vec::new(),
                instructions: Vec::new(),
            }
        }

        pub fn push_instruction(&mut self, instruction: &AgentInstruction) {
            self.instructions.push(instruction.clone());
        }
    }

    #[derive(Clone, Debug)]
    pub struct Listener {
        pub id: u64,
        pub details: String,
    }

    pub struct State {
        pub agents: HashMap<u64, Agent>,
        pub listeners: Vec<Listener>,
    }

    impl State {
        pub fn new() -> Self {
            Self {
                agents: HashMap::new(),
                listeners: Vec::new(),
            }
        }

        pub fn add_agent(&mut self, agent: Agent) {
            self.agents.insert(agent.id, agent);
        }
    }

    pub type SharedState = Arc<RwLock<State>>;

    fn current_time() -> u64 {
        // Mock function to return a timestamp
        0
    }

    // Asynchronous command execution for all agents
    pub async fn push_command(shared_state: &SharedState, command: &str) {
        let mut state = shared_state.write().await;

        for (_, agent) in &mut state.agents {
            println!("Executing command '{}' on Agents...", command);

            let instruction = AgentInstruction {
                packet_header: PacketHeader {
                    agent_id: agent.id,
                    timestamp: current_time(),
                    packet_id: 0, // TODO: Generate a unique packet ID
                    os: None,
                },
                packet_body: AgentInstructionBody::Command {
                    command_id: 0, // TODO: Replace with unique ID generation logic
                    command: command.into(),
                    args: vec![],
                },
            };

            agent.push_instruction(&instruction);
            println!("Command sent successfully to Agent {}.", agent.id);
        }
    }

    // Display active listeners
    pub async fn show_status(shared_state: &SharedState) {
        let state = shared_state.read().await;

        if state.listeners.is_empty() {
            println!("No active listeners.");
        } else {
            println!("Active listeners:");
            for listener in &state.listeners {
                println!("  - {}", listener.details);
            }
        }
    }

    // Display command history
    pub async fn show_history(rl: &Editor<(), FileHistory>) {
        let history = rl.history();
        if history.is_empty() {
            println!("No active history.");
        } else {
            println!("History:");
            for entry in history.iter() {
                println!("  - {}", entry);
            }
        }
    }

    pub async fn execute_command(shared_state: &SharedState, agent_id: u64, command: &str) {
        let mut state = shared_state.write().await;
    
        if let Some(agent) = state.agents.get_mut(&agent_id) {
            println!("Executing command '{}' on Agent {}...", command, agent.id);
    
            let instruction = AgentInstruction {
                packet_header: PacketHeader {
                    agent_id: agent.id,
                    timestamp: current_time(),
                    packet_id: 0, // TODO: Generate a unique packet ID
                    os: agent.os.clone(),
                },
                packet_body: AgentInstructionBody::Command {
                    command_id: 0, // TODO: Replace with unique ID generation logic
                    command: command.into(),
                    args: vec![],  // For now, no arguments are passed
                },
            };
    
            agent.push_instruction(&instruction);
            println!("Command sent successfully to Agent {}.", agent.id);
        } else {
            println!("Agent with ID {} not found.", agent_id);
        }
    }

    // List all registered agents
    pub async fn list_agents(shared_state: &SharedState) {
        let state = shared_state.read().await;

        if state.agents.is_empty() {
            println!("No registered agents.");
        } else {
            println!("Registered agents:");
            for (_, agent) in &state.agents {
                println!(
                    "  - ID: {}, OS: {:?}, IP: {}, Last Response: {}s ago",
                    agent.id,
                    agent.os,
                    agent.ip,
                    current_time() - agent.last_packet_recv
                );
            }
        }
    }
}
