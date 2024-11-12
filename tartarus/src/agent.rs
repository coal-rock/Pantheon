use std::time::SystemTime;
use talaria::{AgentInstruction, AgentInstructionBody, AgentResponse, PacketHeader};

#[post("/monolith", data = "<input>")]
fn monolith(input: Vec<u8>) -> Vec<u8> {
    let response = AgentResponse::deserialize(&input);
    println!("{:#?}", response);

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    return AgentInstruction::serialize(&AgentInstruction {
        packet_header: PacketHeader {
            agent_id: response.packet_header.agent_id,
            timestamp: time,
            packet_id: response.packet_header.packet_id,
        },
        instruction: AgentInstructionBody::Ok,
    });
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![monolith]
}
