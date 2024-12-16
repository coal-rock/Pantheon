use rocket::form::validate::Len;
use serde::Serialize;
use std::{net::SocketAddr, time::SystemTime};
use talaria::{
    AgentInstruction, AgentInstructionBody, AgentResponse, AgentResponseBody, PacketHeader,
};

use crate::SharedState;

#[derive(Serialize, Clone, Debug)]
pub struct Agent {
    nickname: Option<String>,
    id: u64,
    os: Option<String>,
    ip: SocketAddr,
    // time of last response as indicated by agent
    last_response_send: u64,
    // time of last response as indicated by server
    last_response_recv: u64,
    instruction_history: Vec<AgentInstruction>,
    response_history: Vec<AgentResponse>,
}

async fn register_or_update(
    state: &rocket::State<SharedState>,
    response: &AgentResponse,
    instruction: &AgentInstruction,
    addr: SocketAddr,
) {
    let agents = state.read().await.agents.clone();
    let header = response.packet_header.clone();

    // update agent if found
    for i in 0..agents.len() {
        if agents[i].id == header.agent_id {
            log::info!(
                "Heartbeat received from Agent {} at {:?}",
                header.agent_id,
                addr,
            );

            state.write().await.agents[i].last_response_send = header.timestamp;
            state.write().await.agents[i].last_response_recv = time();

            state.write().await.agents[i]
                .response_history
                .push(response.clone());

            state.write().await.agents[i]
                .instruction_history
                .push(instruction.clone());

            return;
        }
    }

    // else add new agent
    let agent = Agent {
        nickname: None,
        id: header.agent_id,
        os: header.os,
        ip: addr,
        last_response_send: header.timestamp,
        last_response_recv: time(),
        instruction_history: vec![],
        response_history: vec![],
    };
    state.write().await.agents.push(agent);
}

#[post("/monolith", data = "<input>")]
pub async fn monolith(
    state: &rocket::State<SharedState>,
    remote_addr: SocketAddr,
    input: Vec<u8>,
) -> Vec<u8> {
    let response = AgentResponse::deserialize(&input);

    let packet_header = response.packet_header.clone();
    let packet_body = response.packet_body.clone();

    let instruction = match packet_body {
        _ => {
            println!("{:#?}", response);

            AgentInstruction {
                packet_header: PacketHeader {
                    agent_id: response.packet_header.agent_id,
                    timestamp: time(),
                    packet_id: response.packet_header.packet_id,
                    os: None,
                },
                instruction: AgentInstructionBody::Ok,
            }
        }
    };

    register_or_update(state, &response, &instruction, remote_addr).await;
    return AgentInstruction::serialize(&instruction);
}

fn time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![monolith]
}
