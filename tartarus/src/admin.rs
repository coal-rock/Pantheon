use rocket::serde::json::Json;
use serde::Serialize;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Serialize, Clone, Debug)]
pub struct AgentStatus {
    pub id: String,
    pub os: String,
    pub ip: String,
    pub active: bool,
}

// Shared list of agents
lazy_static! {
    static ref AGENTS: Mutex<Vec<AgentStatus>> = Mutex::new(Vec::new());
}

// Function to get a copy of the current agents
pub fn list_agents() -> Vec<AgentStatus> {
    AGENTS.lock().unwrap().clone()
}

// Function to add an agent to the list
pub fn add_agent(agent: AgentStatus) {
    let mut agents = AGENTS.lock().unwrap();
    agents.push(agent.clone());
    println!("Agent {} registered.", agent.id);
}

// Rocket route to get agents
#[get("/api/agents")]
pub fn get_agents() -> Json<Vec<AgentStatus>> {
    Json(list_agents())
}

// Route registration
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_agents]
}
