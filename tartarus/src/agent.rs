use crate::SharedState;
use std::net::SocketAddr;
use talaria::helper::current_time;
use talaria::{devlog, protocol::*};

#[post("/monolith", data = "<input>")]
pub async fn monolith(
    state: &rocket::State<SharedState>,
    remote_addr: SocketAddr,
    input: Vec<u8>,
) -> Vec<u8> {
    let response = AgentResponse::deserialize(&input).unwrap();
    let current_time = current_time();

    let instruction_body = {
        let mut state = state.write().await;

        state.try_register_agent(&response, &remote_addr);

        let agent = state.get_agent_mut(&response.header.agent_id);

        match agent {
            Some(agent) => {
                agent.last_packet_send = response.header.timestamp;
                agent.last_packet_recv = current_time;
            }
            None => {}
        }

        state.pop_instruction(&response.header.agent_id)
    };

    let (packet_id, instruction_body) = match instruction_body {
        Some(instruction_body) => (Some(state.write().await.gen_packet_id()), instruction_body),
        None => (None, AgentInstructionBody::Ok),
    };

    let instruction = AgentInstruction {
        header: InstructionHeader {
            packet_id,
            timestamp: current_time,
        },
        body: instruction_body,
    };

    devlog!(
        "\n[{}] {} -> ",
        response
            .header
            .packet_id
            .map_or(" ".to_string(), |num| num.to_string()),
        response.body.variant()
    );

    devlog!(
        "[{}] {} <- ",
        instruction
            .header
            .packet_id
            .map_or(" ".to_string(), |num| num.to_string()),
        instruction.body.variant()
    );

    devlog!(
        "({:#?})",
        state
            .read()
            .await
            .get_network_history(&response.header.agent_id, 10)
            .unwrap()
    );

    let instruction_serialized = AgentInstruction::serialize(&instruction).unwrap();

    {
        let mut state = state.write().await;

        state.statistics.log_send(instruction_serialized.len());
        state.statistics.log_recv(input.len());

        match packet_id {
            Some(_) => {
                state.push_instruction_to_history(&instruction, &response.header.agent_id);
            }
            None => {}
        }

        match response.header.packet_id {
            Some(_) => state.push_response_to_history(&response, &response.header.agent_id),
            None => {}
        }
    }

    instruction_serialized
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![monolith]
}
