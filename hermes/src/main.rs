mod agent;
mod network;
mod commands;

use tokio;

#[tokio::main]
async fn main() {
    // Initialize the agent, which connects to the backend server
    if let Err(e) = network::connect_and_listen().await {
        eprintln!("Failed to connect to server: {}", e);
    }
}
