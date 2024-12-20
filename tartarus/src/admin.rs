use std::collections::HashMap;

use rocket::serde::json::Json;

use crate::SharedState;
use talaria::api::*;

// gets all information about agents
#[get("/api/agents")]
pub async fn get_agents(state: &rocket::State<SharedState>) -> Json<HashMap<u64, Agent>> {
    Json(state.read().await.agents.clone())
}

// gets basic info about agents (name, id, ip, status, ping)
#[get("/api/list_agents")]
pub async fn list_agents(state: &rocket::State<SharedState>) -> Json<Vec<AgentInfo>> {
    let agents: HashMap<u64, Agent> = state.read().await.agents.clone();
    let mut agent_info: Vec<AgentInfo> = vec![];

    for (_, agent) in agents {
        agent_info.push(AgentInfo {
            name: agent.nickname.unwrap_or("No Name".to_string()),
            id: agent.id,
            ip: agent.ip.to_string(),
            status: true,
            ping: agent.last_packet_send - agent.last_packet_recv,
        });
    }

    Json(agent_info)
}

// Route registration
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_agents, list_agents]
}
