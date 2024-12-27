use crate::SharedState;
use talaria::console::*;

pub async fn evaluate_command(
    state: &SharedState,
    command_context: CommandContext,
) -> ConsoleResponse {
    match command_context.command {
        Command::Connect { agent } => connect(state, agent).await,
        _ => todo!(),
    }
}

pub async fn connect(state: &SharedState, target: TargetIdentifier) -> ConsoleResponse {
    let state = state.write().await;

    let success: bool;
    let output: String;

    match target {
        TargetIdentifier::Group { group } => {
            if state.groups.contains_key(&group) {
                success = true;
                output = format!("Successfully connected to group: {}", group);
            } else {
                success = false;
                output = format!("Group {} not found", group);
            }
        }
        TargetIdentifier::Agent { agent } => match agent {
            AgentIdentifier::Nickname { nickname } => {
                if state.nicknames.contains_key(&nickname) {
                    success = true;
                    output = format!(
                        "Succesfully connected to agent: {} [{}]",
                        nickname,
                        state.nicknames.get(&nickname).unwrap()
                    )
                } else {
                    success = false;
                    output = format!("Agent with nickname \"{}\" not found", nickname);
                }
            }
            AgentIdentifier::ID { id } => {
                if state.agents.contains_key(&id) {
                    success = true;
                    output = format!("Succesfully connected to agent: {}", id);
                } else {
                    success = false;
                    output = format!("Agent {} not found", id);
                }
            }
        },
    }

    ConsoleResponse { success, output }
}
