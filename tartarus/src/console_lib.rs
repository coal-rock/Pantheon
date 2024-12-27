use crate::SharedState;
use env_logger::Target;
use rocket::form::validate::with;
use talaria::console::*;

pub async fn evaluate_command(
    state: &SharedState,
    command_context: CommandContext,
) -> ConsoleResponse {
    match command_context.command {
        Command::Connect { agent } => connect(state, agent, command_context.current_target).await,
        Command::Disconnect => disconnect(command_context.current_target).await,
        Command::CreateGroup { group_name, agents } => {
            create_group(state, group_name, agents).await
        }
        Command::DeleteGroup { group_name } => todo!(),
        Command::AddAgentsToGroup { group_name, agents } => todo!(),
        Command::RemoveAgentsFromGroup { group_name, agents } => todo!(),
        Command::Exec { agents, command } => todo!(),
        Command::ListAgents => list_agents(state).await,
        Command::Ping { agents } => todo!(),
        Command::Status { agents } => todo!(),
        Command::Nickname { agent, new_name } => todo!(),
        Command::Clear => todo!(),
    }
}

async fn connect(
    state: &SharedState,
    target: TargetIdentifier,
    current_target: Option<TargetIdentifier>,
) -> ConsoleResponse {
    let state = state.read().await;

    let success: bool;
    let output: String;
    let mut new_target = NewTarget::NoChange;

    if current_target.is_some() {
        return ConsoleResponse {
            success: false,
            output: format!("already connected"),
            new_target: NewTarget::NoChange,
        };
    }

    match target {
        TargetIdentifier::Group { ref group } => {
            if state.groups.contains_key(group) {
                success = true;
                output = format!("successfully connected to group: {}", group);
            } else {
                success = false;
                output = format!("group {} not found", group);
            }
        }
        TargetIdentifier::Agent { ref agent } => match agent {
            AgentIdentifier::Nickname { nickname } => {
                if state.nicknames.contains_key(nickname) {
                    success = true;
                    output = format!(
                        "succesfully connected to agent: {} [{}]",
                        nickname,
                        state.nicknames.get(nickname).unwrap()
                    )
                } else {
                    success = false;
                    output = format!("agent with nickname \"{}\" not found", nickname);
                }
            }
            AgentIdentifier::ID { ref id } => {
                if state.agents.contains_key(id) {
                    success = true;
                    output = format!("succesfully connected to agent: {}", id);
                } else {
                    success = false;
                    output = format!("agent {} not found", id);
                }
            }
        },
    }

    if success {
        new_target = NewTarget::Target { target };
    }

    ConsoleResponse {
        success,
        output,
        new_target,
    }
}

async fn disconnect(current_target: Option<TargetIdentifier>) -> ConsoleResponse {
    match &current_target {
        Some(_) => ConsoleResponse {
            success: true,
            output: format!("successfully disconnected"),
            new_target: NewTarget::NoTarget,
        },
        None => ConsoleResponse {
            success: false,
            output: format!("not currently connected"),
            new_target: NewTarget::NoChange,
        },
    }
}

async fn create_group(
    state: &SharedState,
    group_name: String,
    agents: Vec<AgentIdentifier>,
) -> ConsoleResponse {
    let state = state.write().await;

    if state.groups.contains_key(&group_name) {
        return ConsoleResponse {
            success: false,
            output: format!("group already exists"),
            new_target: NewTarget::NoChange,
        };
    }

    for agent in agents {
        match agent {
            AgentIdentifier::Nickname { nickname } => {
                if !state.nicknames.contains_key(&nickname) {
                    return ConsoleResponse {
                        success: false,
                        output: format!("agent: \"{}\" not found", nickname),
                        new_target: NewTarget::NoChange,
                    };
                }
            }
            AgentIdentifier::ID { id } => {
                if !state.agents.contains_key(&id) {
                    return ConsoleResponse {
                        success: false,
                        output: format!("agent {} not found", id),
                        new_target: NewTarget::NoChange,
                    };
                }
            }
        }
    }

    return ConsoleResponse {
        success: true,
        output: format!("successfully created group"),
        new_target: NewTarget::NoChange,
    };
}

async fn list_agents(state: &SharedState) -> ConsoleResponse {
    let state = state.read().await;
    let mut output = String::new();

    for agent in &state.agents {
        output.push_str(&agent.0.to_string());
    }

    ConsoleResponse {
        success: true,
        output,
        new_target: NewTarget::NoChange,
    }
}
