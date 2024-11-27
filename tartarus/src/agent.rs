use serde::Serialize;
use std::time::SystemTime;
use talaria::{
    AgentInstruction, AgentInstructionBody, AgentResponse, AgentResponseBody, PacketHeader,
};

use crate::SharedState;

#[derive(Serialize, Clone, Debug)]
pub struct Agent {
    id: u64,
    os: Option<String>,
    ip: Option<String>,
    last_ping: u64,
}

impl From<PacketHeader> for Agent {
    fn from(header: PacketHeader) -> Self {
        Agent {
            id: header.agent_id,
            os: header.os,
            ip: header.ip,
            last_ping: header.timestamp,
        }
    }
}

async fn register_agent_if_needed(state: &rocket::State<SharedState>, agent: Agent) {
    let agents = state.read().await.agents.clone();

    if !agents.iter().any(|agent| agent.id == agent.id) {
        state.write().await.agents.push(agent.clone());
    }

    log::info!(
        "Heartbeat received from Agent {} at {:?}",
        agent.id,
        agent.ip
    );
}

#[post("/monolith", data = "<input>")]
pub async fn monolith(state: &rocket::State<SharedState>, input: Vec<u8>) -> Vec<u8> {
    let response = AgentResponse::deserialize(&input);

    match response.packet_body {
        AgentResponseBody::Heartbeat => {
            register_agent_if_needed(state, Agent::from(response.packet_header.clone())).await;
        }
        _ => println!("{:#?}", response),
    }

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    AgentInstruction::serialize(&AgentInstruction {
        packet_header: PacketHeader {
            agent_id: response.packet_header.agent_id,
            timestamp: time,
            packet_id: response.packet_header.packet_id,
            os: None,
            ip: None,
        },
        instruction: AgentInstructionBody::Ok,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![monolith]
}
