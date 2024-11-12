use lazy_static::lazy_static;
use rocket::serde::json::Json;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize, Clone)]
struct AgentStatus {
    id: String,
    os: String,
    ip: String,
    active: bool,
}

lazy_static! {
    static ref AGENTS: Mutex<Vec<AgentStatus>> = Mutex::new(Vec::new());
}

#[get("/agents")]
fn list_agents() -> Json<Vec<AgentStatus>> {
    let agents = AGENTS.lock().unwrap().clone();
    Json(agents)
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_agents]
}
