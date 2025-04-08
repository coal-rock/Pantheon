use std::net::SocketAddr;

use talaria::helper::current_time;
use talaria::protocol::*;

use crate::SharedState;

#[post("/monolith", data = "<input>")]
pub async fn monolith(
    state: &rocket::State<SharedState>,
    remote_addr: SocketAddr,
    input: Vec<u8>,
) -> Vec<u8> {
    let response = AgentResponse::deserialize(&input).unwrap();

    state
        .write()
        .await
        .try_register_agent(&response, &remote_addr);

    let instruction_body = state
        .write()
        .await
        .pop_instruction(&response.header.agent_id);

    let (packet_id, instruction_body) = match instruction_body {
        Some(instruction_body) => (Some(state.write().await.gen_packet_id()), instruction_body),
        None => (None, AgentInstructionBody::Ok),
    };

    let instruction = AgentInstruction {
        header: InstructionHeader {
            packet_id,
            timestamp: current_time(),
        },
        body: instruction_body,
    };

    println!(
        "[{}] {} -> ",
        response.header.packet_id.unwrap_or(50),
        response.body.variant()
    );

    println!(
        "[{}] {} <- ",
        instruction.header.packet_id.unwrap_or(50),
        instruction.body.variant()
    );

    let instruction = AgentInstruction::serialize(&instruction).unwrap();

    {
        let mut state = state.write().await;

        state.statistics.log_send(instruction.len());
        state.statistics.log_recv(input.len());
    }

    instruction
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![monolith]
}
