use crate::console_lib;
use crate::SharedState;
use rustyline::{error::ReadlineError, history::FileHistory, Editor};
use std::time::SystemTime;
use talaria::console::*;

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

    let mut console = Console::new(None);

    loop {
        let readline = rl.readline(&console.status_line());
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let command_str = line.trim();
                let command = console.handle_command(command_str.to_string());

                match command {
                    Ok(command) => {
                        println!("{:#?}", command);

                        let output = console_lib::evaluate_command(
                            shared_state,
                            CommandContext {
                                command,
                                current_target: console.get_target(),
                            },
                        )
                        .await;

                        match output.new_target {
                            NewTarget::NoTarget => console.set_target(None),
                            NewTarget::Target { ref target } => {
                                console.set_target(Some(target.clone()))
                            }
                            NewTarget::NoChange => {}
                        }

                        println!("{:#?}", output);
                    }
                    Err(error) => println!("{}", error.to_string()),
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
    println!(
        "  exec         - Execute a command on a specific agent (Usage: exec <agent_id> <command>)"
    );
    println!("  exit         - Exit the Tartarus Console");
    println!("  help         - Show this help message");
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
