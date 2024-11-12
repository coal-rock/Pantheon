mod agent;
mod network;

use agent::Agent;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // let backend_server_addr =
    //     env::var("BACKEND_SERVER_ADDR").expect("BACKEND_SERVER_ADDR must be set in .env");
    let backend_server_addr = "localhost:5000";

    let mut agent = Agent::new(backend_server_addr.to_string());

    loop {
        match network::send_heartbeat(&mut agent).await {
            Some(instruction) => network::handle_response(&mut agent, instruction).await,
            None => println!("bleh"),
        }

        sleep(Duration::from_millis(agent.polling_interval_millis)).await;
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
}
