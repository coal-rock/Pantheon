#[macro_use]
extern crate rocket;
extern crate log;

mod admin;
mod agent;
mod console;

use rocket::{Rocket, Build, Ignite};
use rocket::tokio::sync::RwLock;
use std::sync::Arc;
use crate::console::start_console;

// Shared state for active listeners
#[derive(Default)]
struct ActiveListeners {
    listeners: Vec<String>,
}

// Wrap in Arc and RwLock for safe concurrent access
type SharedState = Arc<RwLock<ActiveListeners>>;

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

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    // Initialize logger
    env_logger::init();

    // Initialize shared state for active listeners
    let shared_state = Arc::new(RwLock::new(ActiveListeners::default()));

    // Launch the Rocket server in a separate task
    tokio::spawn({
        let shared_state = shared_state.clone();
        async move {
            launch_rocket(shared_state).await.unwrap();
        }
    });

    // Start the interactive console
    start_console(shared_state).await;

    Ok(())
}
