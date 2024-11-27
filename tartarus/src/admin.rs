use rocket::serde::json::Json;

use crate::{agent::Agent, SharedState};

// Rocket route to get agents
#[get("/api/agents")]
pub async fn get_agents(state: &rocket::State<SharedState>) -> Json<Vec<Agent>> {
    Json(state.read().await.agents.clone())
}

// Route registration
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_agents]
}
