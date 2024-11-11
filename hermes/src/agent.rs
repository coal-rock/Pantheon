use std::process::Command;
use sys_info::{os_type, hostname};
use local_ip_address::local_ip;

pub struct SystemInfo {
    pub os: String,
    pub ip: String,
}

pub async fn get_system_info() -> Result<SystemInfo, Box<dyn std::error::Error>> {
    let os = os_type()?;
    let ip = local_ip()?.to_string();
    Ok(SystemInfo { os, ip })
}

pub fn execute_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// Optional: Add functions here for starting an interactive shell session, etc.
