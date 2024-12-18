use rocket::serde::json::Json;

use crate::SharedState;
use talaria::{Agent, AgentInfo};

// gets all information about agents
#[get("/api/agents")]
pub async fn get_agents(state: &rocket::State<SharedState>) -> Json<Vec<Agent>> {
    Json(state.read().await.agents.clone())
}

// gets basic info about agents (name, id, ip, status, ping)
#[get("/api/list_agents")]
pub async fn list_agents(state: &rocket::State<SharedState>) -> Json<Vec<AgentInfo>> {
    let agents: Vec<Agent> = state.read().await.agents.clone();
    let mut agent_info: Vec<AgentInfo> = vec![];

    for agent in agents {
        agent_info.push(AgentInfo {
            name: agent.nickname.unwrap_or("No Name".to_string()),
            id: agent.id,
            ip: agent.ip.to_string(),
            status: true,
            ping: agent.last_response_send - agent.last_response_recv,
        });
    }

    Json(agent_info)
}

// Route registration
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_agents, list_agents]
}
