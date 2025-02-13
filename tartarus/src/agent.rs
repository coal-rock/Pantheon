use serde::Serialize;
use std::{net::SocketAddr, time::SystemTime};

use talaria::api::*;
use talaria::protocol::*;

use crate::SharedState;

// Register or update agent in the state
async fn register_or_update(
    state: &rocket::State<SharedState>,
    response: &AgentResponse,
    instruction: &AgentInstruction,
    addr: SocketAddr,
) {
    let mut state = state.write().await;
    let agent_id = response.packet_header.agent_id;

    if state.agents.contains_key(&agent_id) {
        // update agent if found
        let agent = state.agents.get_mut(&agent_id).unwrap();
        log::info!("Updated Agent {} at {:?}", agent.id, addr);
        agent.last_packet_send = response.packet_header.timestamp;
        agent.last_packet_recv = current_time();
        agent.push_response(response);
        agent.push_instruction(instruction);
        return;
    } else {
        // add new agent if not found
        state.agents.insert(
            response.packet_header.agent_id,
            Agent {
                nickname: None,
                id: response.packet_header.agent_id,
                os: response.packet_header.os.clone(),
                ip: addr,
                last_packet_send: response.packet_header.timestamp,
                last_packet_recv: current_time(),
                network_history: vec![
                    NetworkHistoryEntry::AgentResponse {
                        response: response.clone(),
                    },
                    NetworkHistoryEntry::AgentInstruction {
                        instruction: instruction.clone(),
                    },
                ],
                queue: vec![],
            },
        );
    }
}

// Route to handle agent responses and issue instructions
#[post("/monolith", data = "<input>")]
pub async fn monolith(
    state: &rocket::State<SharedState>,
    remote_addr: SocketAddr,
    input: Vec<u8>,
) -> Vec<u8> {
    panic!();
    let response = AgentResponse::deserialize(&input).unwrap();
    let packet_body = response.packet_body.clone();

    // Generate an instruction based on the received response
    let instruction = match packet_body {
        AgentResponseBody::CommandResponse {
            command: _,
            command_id: _,
            status_code: _,
            stdout,
            stderr,
        } => {
            log::info!("Command Output:\nstdout: {}\nstderr: {}", stdout, stderr);

            AgentInstruction {
                packet_header: PacketHeader {
                    agent_id: response.packet_header.agent_id,
                    timestamp: current_time(),
                    packet_id: response.packet_header.packet_id,
                    os: None,
                },
                packet_body: AgentInstructionBody::Ok,
            }
        }
        AgentResponseBody::Heartbeat => {
            let mut state = state.write().await;

            let agent = state.agents.get_mut(&response.packet_header.agent_id);

            if agent.is_none() {
                AgentInstruction {
                    packet_header: PacketHeader {
                        agent_id: response.packet_header.agent_id,
                        timestamp: current_time(),
                        packet_id: response.packet_header.packet_id,
                        os: None,
                    },
                    packet_body: AgentInstructionBody::Ok,
                }
            } else {
                let agent = agent.unwrap();
                let body = agent.pop_instruction();

                AgentInstruction {
                    packet_header: PacketHeader {
                        agent_id: response.packet_header.agent_id,
                        timestamp: current_time(),
                        packet_id: response.packet_header.packet_id,
                        os: None,
                    },
                    packet_body: body.unwrap_or(AgentInstructionBody::Ok),
                }
            }
        }
        _ => AgentInstruction {
            packet_header: PacketHeader {
                agent_id: response.packet_header.agent_id,
                timestamp: current_time(),
                packet_id: response.packet_header.packet_id,
                os: None,
            },
            packet_body: AgentInstructionBody::Command {
                command_id: 1, // Example command_id; replace with logic for unique IDs
                command: "echo".into(),
                args: vec!["Hello from server!".into()],
            },
        },
    };

    // Update agent state
    register_or_update(state, &response, &instruction, remote_addr).await;

    // respond to agent with instruction
    AgentInstruction::serialize(&instruction).unwrap()
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
