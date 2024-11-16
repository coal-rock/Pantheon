use crate::admin::{add_agent, list_agents, AgentStatus};
use talaria::{AgentInstruction, AgentInstructionBody, AgentResponse, PacketHeader, AgentResponseBody};
use std::time::SystemTime;

fn register_agent_if_needed(agent_id: u64, os: &str, ip: &str) {
    let agent_id_str = agent_id.to_string();

    let agents = list_agents();
    if !agents.iter().any(|agent| agent.id == agent_id_str) {
        add_agent(AgentStatus {
            id: agent_id_str.clone(),
            os: os.to_string(),
            ip: ip.to_string(),
            active: true,
        });
    }

    log::info!("Heartbeat received from Agent {} at {}", agent_id, ip);
}

#[post("/monolith", data = "<input>")]
fn monolith(input: Vec<u8>) -> Vec<u8> {
    let response = AgentResponse::deserialize(&input);

    match response.response {
        AgentResponseBody::Heartbeat => {
            register_agent_if_needed(response.packet_header.agent_id, "OS Placeholder", "IP Placeholder");
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
        },
        instruction: AgentInstructionBody::Ok,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![monolith]
}
