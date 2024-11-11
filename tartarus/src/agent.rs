use rocket::Route;
use rocket::serde::json::Json;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

#[derive(Serialize, Clone)]
struct AgentStatus {
    id: u8,
    os: String,
    ip: String,
    active: bool,
}

lazy_static! {
    static ref AGENTS: Arc<Mutex<HashMap<u8, AgentStatus>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[post("/<agent_id>/update")]
fn update_agent(agent_id: u8) -> String {
    format!("Updating agent with ID: {}", agent_id)
}

#[post("/<agent_id>/deactivate")]
fn deactivate_agent(agent_id: u8) -> String {
    if let Ok(mut agents) = AGENTS.lock() {
        if let Some(agent) = agents.get_mut(&agent_id) {
            agent.active = false;
            return format!("Deactivated agent with ID: {}", agent_id);
        }
    }
    format!("Agent with ID: {} not found", agent_id)
}

#[post("/<agent_id>/activate")]
fn activate_agent(agent_id: u8) -> String {
    if let Ok(mut agents) = AGENTS.lock() {
        if let Some(agent) = agents.get_mut(&agent_id) {
            agent.active = true;
            return format!("Activated agent with ID: {}", agent_id);
        }
    }
    format!("Agent with ID: {} not found", agent_id)
}

#[get("/<agent_id>/get_file?<file_id>")]
fn retrieve_file(agent_id: u8, file_id: u8) -> String {
    format!("Retrieving file {} from agent {}", file_id, agent_id)
}

#[get("/<agent_id>/list_files")]
fn list_files(agent_id: u8) -> String {
    format!("Listing files for agent {}", agent_id)
}

#[post("/<agent_id>/uninstall")]
fn uninstall_agent(agent_id: u8) -> String {
    if let Ok(mut agents) = AGENTS.lock() {
        agents.remove(&agent_id);
        return format!("Uninstalled agent with ID: {}", agent_id);
    }
    format!("Agent with ID: {} not found", agent_id)
}

#[get("/agents")]
fn list_agents() -> Json<Vec<AgentStatus>> {
    let agents = AGENTS.lock().unwrap().values().cloned().collect::<Vec<_>>();
    Json(agents)
}

#[get("/log")]
fn get_log() -> String {
    "Retrieving server logs...".to_string()
}

#[get("/<agent_id>/log")]
fn get_log_agent(agent_id: u8) -> String {
    format!("Retrieving logs for agent {}", agent_id)
}

#[post("/escape_hatch")]
fn escape_hatch() -> String {
    "Escape hatch triggered!".to_string()
}

pub fn routes() -> Vec<Route> {
    routes![
        update_agent,
        deactivate_agent,
        activate_agent,
        retrieve_file,
        list_files,
        uninstall_agent,
        list_agents,
        get_log,
        get_log_agent,
        escape_hatch,
    ]
}
