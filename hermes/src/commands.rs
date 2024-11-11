use serde_json::Value;
use crate::agent;

pub fn handle_command(command_json: &str) -> Option<String> {
    let command: Value = serde_json::from_str(command_json).ok()?;

    // Check the command type
    if let Some(action) = command["action"].as_str() {
        match action {
            "execute" => {
                if let Some(cmd) = command["command"].as_str() {
                    match agent::execute_command(cmd) {
                        Ok(output) => Some(output),
                        Err(e) => Some(format!("Error executing command: {}", e)),
                    }
                } else {
                    Some("Invalid command format".to_string())
                }
            }
            "shell" => {
                // Placeholder for interactive shell
                Some("Shell requested, but not implemented yet".to_string())
            }
            _ => Some("Unknown action".to_string())
        }
    } else {
        None
    }
}
