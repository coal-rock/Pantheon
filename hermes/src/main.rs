use talaria::client::{send_request, Request};
use dotenv::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
struct SystemInfo {
    id: String,
    os: String,
    ip: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let backend_server_addr = env::var("BACKEND_SERVER_ADDR")
        .expect("BACKEND_SERVER_ADDR must be set in .env");

    let system_info = SystemInfo {
        id: "Agent-1234".to_string(),
        os: "Linux".to_string(),
        ip: "dynamic-ip-address".to_string(),
    };

    let request = Request {
        agent_id: system_info.id,
        action: "heartbeat".to_string(),
        payload: None,
    };

    match send_request(&backend_server_addr, request).await {
        Ok(response) => println!("Response: {}", response.status),
        Err(e) => eprintln!("Error: {}", e),
    }
}
