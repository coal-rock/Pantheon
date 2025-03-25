use crate::console_lib;
use crate::SharedState;
use rustyline::{error::ReadlineError, history::FileHistory, Editor};
use talaria::console::*;

pub async fn start_console(shared_state: &SharedState) {
    let mut rl = Editor::<(), FileHistory>::new().unwrap();

    // Load command history if it exists
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut console = Console::new(None);

    print!("\x1B[2J\x1B[1;1H");
    println!("Type 'help' for a list of commands.");

    loop {
        let readline = rl.readline(&console.status_line());
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let command_str = line.trim();
                let command = console.handle_command(command_str.to_string());

                match command {
                    Ok(command) => {
                        let response = console_lib::evaluate_command(
                            shared_state,
                            CommandContext {
                                command,
                                current_target: console.get_target(),
                            },
                        )
                        .await;

                        match response {
                            Ok(response) => {
                                match response.new_target {
                                    NewTarget::NoTarget => console.set_target(None),
                                    NewTarget::Target { ref target } => {
                                        console.set_target(Some(target.clone()))
                                    }
                                    NewTarget::NoChange => {}
                                }

                                println!("{}", response.output);
                            }
                            Err(err) => {
                                println!("{}", err.message);
                            }
                        }
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
