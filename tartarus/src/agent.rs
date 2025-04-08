use std::net::SocketAddr;

use talaria::api::*;
use talaria::helper::current_time;
use talaria::protocol::*;

use crate::SharedState;

// Register or update egent in the state
async fn register_or_update(
    state: &rocket::State<SharedState>,
    response: &AgentResponse,
    instruction: &AgentInstruction,
    addr: SocketAddr,
) {
    let mut state = state.write().await;
    let agent_id = response.header.agent_id;

    if state.agents.contains_key(&agent_id) {
        // update agent if found
        let config = state.config.clone();
        let agent = state.agents.get_mut(&agent_id).unwrap();
        log::info!("Updated Agent {} at {:?}", agent.id, addr);
        agent.last_packet_send = response.header.timestamp;
        agent.last_packet_recv = current_time();
        agent.push_response(response, config.history_buf_len);
        agent.push_instruction(instruction, config.history_buf_len);
        return;
    } else {
        // add new agent if not found
        state.agents.insert(
            response.header.agent_id,
            Agent {
                nickname: None,
                id: response.header.agent_id,
                os: response.header.os.clone(),
                external_ip: addr,
                internal_ip: response.header.internal_ip.clone(),
                last_packet_send: response.header.timestamp,
                last_packet_recv: current_time(),
                network_history: vec![
                    NetworkHistoryEntry::AgentResponse {
                        response: response.clone(),
                    },
                    NetworkHistoryEntry::AgentInstruction {
                        instruction: instruction.clone(),
                    },
                ]
                .into(),
                queue: vec![],
                polling_interval_ms: response.header.polling_interval_ms,
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
    let response = AgentResponse::deserialize(&input).unwrap();
    let packet_body = response.body.clone();

    // Generate an instruction based on the received response
    let instruction = match packet_body {
        AgentResponseBody::CommandResponse {
            command: _,
            status_code: _,
            stdout,
            stderr,
        } => {
            log::info!("Command Output:\nstdout: {}\nstderr: {}", stdout, stderr);

            AgentInstruction {
                header: InstructionHeader { packet_id: 1 },
                body: AgentInstructionBody::Ok,
            }
        }
        AgentResponseBody::Heartbeat => {
            let mut state = state.write().await;

            let agent = state.agents.get_mut(&response.header.agent_id);

            if agent.is_none() {
                AgentInstruction {
                    header: InstructionHeader { packet_id: 1 },
                    body: AgentInstructionBody::Ok,
                }
            } else {
                let agent = agent.unwrap();
                let body = agent.pop_instruction();

                AgentInstruction {
                    header: InstructionHeader { packet_id: 1 },
                    body: body.unwrap_or(AgentInstructionBody::Ok),
                }
            }
        }
        _ => AgentInstruction {
            header: InstructionHeader { packet_id: 1 },
            body: AgentInstructionBody::Command {
                command: "echo".into(),
                args: vec!["Hello from server!".into()],
            },
        },
    };

    // Update agent state
    register_or_update(state, &response, &instruction, remote_addr).await;

    // respond to agent with instruction
    let instruction = AgentInstruction::serialize(&instruction).unwrap();

    let state = &mut state.write().await;
    state.statistics.log_recv(input.len());

    state
        .statistics
        .log_latency(current_time() - response.header.timestamp);

    state.statistics.log_send(instruction.len());

    instruction
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![monolith]
}
