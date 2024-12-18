use crate::SharedState;
use std::io::{self, Write};
use std::time::SystemTime;
use talaria::protocol::*;

pub async fn start_console(shared_state: &SharedState) {
    println!("========================================");
    println!("         Tartarus Command Console       ");
    println!("========================================");
    println!("Type 'help' for a list of commands.\n");

    loop {
        print!("tartarus> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "status" => show_status(shared_state).await,
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
            "exit" => {
                println!("Exiting Tartarus Console.");
                break;
            }
            "help" => show_help(),
            _ => println!("Unknown command. Type 'help' for a list of commands."),
        }
    }
}

fn show_help() {
    println!("Available commands:");
    println!("  status       - Show the status of all active listeners");
    println!("  list agents  - List all registered agents");
    println!("  exec         - Execute a command on a specific agent");
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
