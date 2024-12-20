#[macro_use]
extern crate rocket;
extern crate log;

mod admin;
mod agent;
mod console;
mod served_files;

use crate::console::start_console;
use rocket::tokio::sync::RwLock;
use rocket::{Build, Ignite, Rocket};
use served_files::serve_compiled_file;
use std::collections::HashMap;
use std::sync::Arc;
use talaria::api::*;

// Shared state for active listeners
#[derive(Default)]
struct State {
    listeners: Vec<String>,
    agents: HashMap<u64, Agent>,
}

// Wrap in Arc and RwLock for safe concurrent access
type SharedState = Arc<RwLock<State>>;

// Rocket instance with shared state
fn rocket(shared_state: SharedState) -> Rocket<Build> {
    rocket::build()
        .mount("/admin", admin::routes()) // Admin routes for agent management
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
