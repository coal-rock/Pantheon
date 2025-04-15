use crate::auth::Auth;
use crate::SharedState;
use rocket::serde::json::Json;
use std::collections::HashMap;
use talaria::api::*;

/// Retrieves all agents
#[get("/agents")]
pub async fn get_agents(
    _auth: Auth,
    state: &rocket::State<SharedState>,
) -> Json<HashMap<u64, Agent>> {
    Json(state.read().await.agents.clone())
}

/// Retrieves arbitrary amount of network history
/// for specified agent
#[get("/<agent_id>/network_history?<count>")]
pub async fn get_agent_history(
    _auth: Auth,
    state: &rocket::State<SharedState>,
    agent_id: u64,
    count: usize,
) -> Json<Vec<NetworkHistoryEntry>> {
    Json(state.read().await.get_agent_history(agent_id, count))
}

/// Retrieves basic info about agent
#[get("/list_agents")]
pub async fn list_agents(_auth: Auth, state: &rocket::State<SharedState>) -> Json<Vec<AgentInfo>> {
    Json(state.read().await.get_agent_list())
}

#[get("/tartarus_info")]
pub async fn tartarus_info(_auth: Auth, state: &rocket::State<SharedState>) -> Json<TartarusInfo> {
    Json(state.read().await.get_tartarus_info())
}

#[get("/tartarus_stats")]
pub async fn tartarus_stats(
    _auth: Auth,
    state: &rocket::State<SharedState>,
) -> Json<TartarusStats> {
    Json(state.read().await.get_tartarus_stats())
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_agents,
        list_agents,
        get_agent_history,
        tartarus_info,
        tartarus_stats,
    ]
}
