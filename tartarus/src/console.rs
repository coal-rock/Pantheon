use crate::admin;
use crate::SharedState;
use std::io::{self, Write};

pub async fn start_console(shared_state: SharedState) {
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
            "status" => show_status(&shared_state).await,
            "list agents" => list_agents(&shared_state).await,
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
    println!("  exit         - Exit the Tartarus Console");
    println!("  help         - Show this help message");
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

async fn list_agents(_shared_state: &SharedState) {
    let agents = admin::list_agents();

    if agents.is_empty() {
        println!("No registered agents.");
    } else {
        println!("Registered agents:");
        for agent in agents {
            println!("  - {:?}", agent);
        }
    }
}
