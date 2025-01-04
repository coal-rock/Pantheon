use crate::SharedState;
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
        Command::DeleteGroup { group_name } => delete_group(state, group_name).await,
        Command::AddAgentsToGroup { group_name, agents } => {
            add_agents_to_group(state, group_name, agents).await
        }
        Command::RemoveAgentsFromGroup { group_name, agents } => {
            remove_agents_from_group(state, group_name, agents).await
        }
        Command::Exec { agents, command } => todo!(),
        Command::ListAgents => list_agents(state).await,
        Command::Ping { agents } => todo!(),
        Command::Status { agents } => todo!(),
        Command::Nickname { agent, new_name } => {
            nickname(state, command_context.current_target, agent, new_name).await
        }
        Command::Clear => clear().await,
        Command::ListGroups => list_groups(state).await,
        Command::Help => todo!(),
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
        TargetIdentifier::Agent { ref agent } => match state.get_agent(agent.clone()) {
            Some(agent) => {
                success = true;
                output = format!(
                    "successfully connected to: {} [{}]",
                    agent.id,
                    agent.clone().nickname.unwrap_or("!!!".to_string())
                )
            }
            None => {
                success = false;
                output = format!("agent not found");
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
    let mut state = state.write().await;
    let mut agent_ids: Vec<u64> = vec![];

    if state.groups.contains_key(&group_name) {
        return ConsoleResponse {
            success: false,
            output: format!("group already exists"),
            new_target: NewTarget::NoChange,
        };
    }

    for ident in &agents {
        match state.get_agent(ident.clone()) {
            Some(agent) => agent_ids.push(agent.id),
            None => {
                return ConsoleResponse {
                    success: true,
                    output: format!("agent {:#?} not found", ident),
                    new_target: NewTarget::NoChange,
                }
            }
        }
    }

    agent_ids.dedup();

    state.groups.insert(group_name, agent_ids);

    return ConsoleResponse {
        success: true,
        output: format!("successfully created group"),
        new_target: NewTarget::NoChange,
    };
}

async fn delete_group(state: &SharedState, group_name: String) -> ConsoleResponse {
    let mut state = state.write().await;

    match state.groups.remove(&group_name) {
        Some(_) => ConsoleResponse {
            success: true,
            output: format!("successfully deleted group: {}", group_name),
            new_target: NewTarget::NoChange,
        },
        None => ConsoleResponse {
            success: false,
            output: format!("couldn't delete group {}", group_name),
            new_target: NewTarget::NoChange,
        },
    }
}

async fn add_agents_to_group(
    state: &SharedState,
    group_name: String,
    agents: Vec<AgentIdentifier>,
) -> ConsoleResponse {
    let mut state = state.write().await;
    let mut agent_ids: Vec<u64> = vec![];

    if !state.groups.contains_key(&group_name) {
        return ConsoleResponse {
            success: false,
            output: format!("group not found"),
            new_target: NewTarget::NoChange,
        };
    }

    for ident in agents {
        match state.get_agent(ident.clone()) {
            Some(agent) => agent_ids.push(agent.id),
            None => {
                return ConsoleResponse {
                    success: false,
                    output: format!("agent {:#?} not found", ident),
                    new_target: NewTarget::NoChange,
                }
            }
        }
    }

    agent_ids.dedup();

    state
        .groups
        .get_mut(&group_name)
        .unwrap()
        .append(&mut agent_ids);

    state.groups.get_mut(&group_name).unwrap().dedup();

    ConsoleResponse {
        success: true,
        output: format!("successfully added agents to group"),
        new_target: NewTarget::NoChange,
    }
}

async fn remove_agents_from_group(
    state: &SharedState,
    group_name: String,
    agents: Vec<AgentIdentifier>,
) -> ConsoleResponse {
    let mut state = state.write().await;
    let mut agent_ids: Vec<u64> = vec![];

    if !state.groups.contains_key(&group_name) {
        return ConsoleResponse {
            success: false,
            output: format!("group not found"),
            new_target: NewTarget::NoChange,
        };
    }

    for ident in agents {
        match state.get_agent(ident.clone()) {
            Some(agent) => agent_ids.push(agent.id),
            None => {
                return ConsoleResponse {
                    success: false,
                    output: format!("agent {:#?} not found", ident),
                    new_target: NewTarget::NoChange,
                }
            }
        }
    }

    agent_ids.dedup();

    for (index, group_member) in state
        .groups
        .get(&group_name)
        .unwrap()
        .clone()
        .into_iter()
        .enumerate()
    {
        for agent_id in &agent_ids {
            if group_member.clone() == *agent_id {
                state.groups.get_mut(&group_name).unwrap().remove(index);
            }
        }
    }

    ConsoleResponse {
        success: true,
        output: format!("succesfully removed agents from group"),
        new_target: NewTarget::NoChange,
    }
}

async fn clear() -> ConsoleResponse {
    ConsoleResponse {
        success: true,
        output: "\033c".to_string(),
        new_target: NewTarget::NoChange,
    }
}

async fn list_agents(state: &SharedState) -> ConsoleResponse {
    let state = state.read().await;
    let mut output = String::new();

    for (id, agent) in &state.agents {
        output.push_str(
            format!(
                "{} - [{}]\n",
                id,
                agent.clone().nickname.unwrap_or(String::from("!!!"))
            )
            .clone()
            .as_str(),
        );
    }

    ConsoleResponse {
        success: true,
        output,
        new_target: NewTarget::NoChange,
    }
}

async fn list_groups(state: &SharedState) -> ConsoleResponse {
    let state = state.read().await;
    let agents = state.agents.clone();
    let mut output = String::new();

    for (group_name, ids) in &state.groups {
        output.push_str(&format!("#{}:\n", group_name));

        for id in ids {
            output.push_str(
                format!(
                    "   {} - [{}]\n",
                    id,
                    agents
                        .get(id)
                        .unwrap()
                        .nickname
                        .clone()
                        .unwrap_or(String::from("!!!"))
                )
                .clone()
                .as_str(),
            );
        }
    }

    ConsoleResponse {
        success: true,
        output,
        new_target: NewTarget::NoChange,
    }
}

async fn nickname(
    state: &SharedState,
    current_target: Option<TargetIdentifier>,
    agent: Option<AgentIdentifier>,
    nickname: String,
) -> ConsoleResponse {
    let mut state = state.write().await;

    if agent.is_some() {
        let agent = agent.unwrap();

        match state.get_agent_mut(agent.clone()) {
            Some(agent) => {
                agent.nickname = Some(nickname);

                return ConsoleResponse {
                    success: true,
                    output: format!("set agent nickname"),
                    new_target: NewTarget::NoChange,
                };
            }
            None => {
                return ConsoleResponse {
                    success: false,
                    output: format!("agent {:#?} not found", agent),
                    new_target: NewTarget::NoChange,
                }
            }
        }
    } else if current_target.is_some() {
        let current_target = match current_target.unwrap() {
            TargetIdentifier::Group { group: _ } => {
                return ConsoleResponse {
                    success: false,
                    output: format!("must be connected to agent"),
                    new_target: NewTarget::NoChange,
                }
            }
            TargetIdentifier::Agent { agent } => agent,
        };

        match state.get_agent_mut(current_target.clone()) {
            Some(agent) => {
                agent.nickname = Some(nickname);

                return ConsoleResponse {
                    success: true,
                    output: format!("set agent nickname"),
                    new_target: NewTarget::NoChange,
                };
            }
            None => {
                return ConsoleResponse {
                    success: false,
                    output: format!("agent {:#?} not found", current_target),
                    new_target: NewTarget::NoChange,
                }
            }
        }
    }

    ConsoleResponse {
        success: false,
        output: format!("must be connected to agent, or agent must be specified"),
        new_target: NewTarget::NoChange,
    }
}
