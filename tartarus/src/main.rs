#[macro_use]
extern crate rocket;
extern crate log;

mod admin;
mod agent;
mod console_interface;
mod console_lib;
mod console_net;
mod served_files;

use crate::console_interface::start_console;
use rocket::tokio::sync::RwLock;
use rocket::{Build, Ignite, Rocket};
use std::collections::HashMap;
use std::sync::Arc;
use talaria::api::*;
use talaria::console::*;

// Shared state for active listeners
#[derive(Default)]
struct State {
    listeners: Vec<String>,
    agents: HashMap<u64, Agent>,
    groups: HashMap<String, Vec<u64>>,
}

impl State {
    pub fn get_agent(&self, ident: AgentIdentifier) -> Option<&Agent> {
        match ident {
            AgentIdentifier::Nickname { nickname } => {
                for (_, agent) in &self.agents {
                    if agent.nickname == Some(nickname.clone()) {
                        return Some(&agent);
                    }
                }
            }
            AgentIdentifier::ID { id } => return self.agents.get(&id),
        }

        return None;
    }

    pub fn get_agent_mut(&mut self, ident: AgentIdentifier) -> Option<&mut Agent> {
        match ident {
            AgentIdentifier::Nickname { nickname } => {
                for (id, agent) in self.agents.clone() {
                    if agent.nickname == Some(nickname.clone()) {
                        return self.agents.get_mut(&id);
                    }
                }
            }
            AgentIdentifier::ID { id } => return self.agents.get_mut(&id),
        }

        return None;
    }
}

// Wrap in Arc and RwLock for safe concurrent access
type SharedState = Arc<RwLock<State>>;

// Rocket instance with shared state
fn rocket(shared_state: SharedState) -> Rocket<Build> {
    rocket::build()
        .mount("/admin", admin::routes()) // Admin routes for agent management
        .mount("/console", console_net::routes()) // Routes for console protocol
        .mount("/agent", agent::routes()) // Agent-specific routes
        .manage(shared_state) // Shared state for listeners
        .mount("/listeners", routes![get_listeners]) // Endpoint to fetch listeners
        .configure(rocket::Config {
            log_level: rocket::config::LogLevel::Critical, // Suppress unnecessary Rocket logs
            ..rocket::Config::default()
        })
}

// Endpoint to fetch active listeners
#[get("/")]
async fn get_listeners(state: &rocket::State<SharedState>) -> String {
    let listeners = state.read().await;
    format!("Active listeners: {:?}", listeners.listeners)
}

// Launch the Rocket server asynchronously
async fn launch_rocket(shared_state: SharedState) -> Result<Rocket<Ignite>, rocket::Error> {
    rocket(shared_state).launch().await
}

fn setup_static_routes() -> Rocket<Build> {
    rocket::build().mount("/", routes![served_files::serve_compiled_file])
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    // Initialize logger
    env_logger::init();

    // Initialize shared state for active listeners
    let shared_state = Arc::new(RwLock::new(State::default()));

    // Launch the Rocket server in a separate task
    tokio::spawn({
        let shared_state = shared_state.clone();
        async move {
            launch_rocket(shared_state).await.unwrap();
        }
    });

    // Start the interactive console
    start_console(&shared_state).await;

    Ok(())
}
