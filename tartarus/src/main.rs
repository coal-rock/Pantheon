#[macro_use]
extern crate rocket;
extern crate log;

mod admin;
mod agent;
mod auth;
mod binaries;
mod config;
mod console_interface;
mod console_lib;
mod console_net;
mod cors;
mod state;
mod statistics;

pub mod helper;

use crate::console_interface::start_console;
use config::Config;
use state::{SharedState, State};

use cors::CORS;
use rocket::{Build, Rocket};
use std::{fs, path::PathBuf};

// Rocket instance with shared state
async fn rocket(shared_state: SharedState) -> Rocket<Build> {
    let config = shared_state.read().await.config.clone();

    rocket::build()
        .mount("/api/admin", admin::routes())
        .mount("/api/admin/console", console_net::routes())
        .mount("/api/agent", agent::routes())
        .mount("/api/binaries", binaries::routes())
        .manage(shared_state)
        .attach(CORS)
        .configure(rocket::Config {
            log_level: rocket::config::LogLevel::Critical,
            address: config.address,
            port: config.port,
            ..rocket::Config::default()
        })
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::init();

    let config_path: PathBuf = PathBuf::from("tartarus.toml");

    let config: Config = match fs::read_to_string(&config_path) {
        Err(_) => {
            println!(
                "Config file not found at: {}",
                config_path.into_os_string().into_string().unwrap()
            );
            println!("Using default values");
            Config::default()
        }
        Ok(config_str) => {
            println!(
                "Config file found at: {}",
                config_path.into_os_string().into_string().unwrap()
            );

            match toml::from_str::<Config>(&config_str) {
                Ok(config) => config,
                Err(_) => {
                    println!("Unable to parse config file");
                    println!("Using default values:\n");
                    Config::default()
                }
            }
        }
    };

    println!("\nConfiguration:");
    println!("--------------------------");
    print!("{}", toml::to_string_pretty(&config.clone()).unwrap());
    println!("--------------------------\n");

    // Initialize shared state for active listeners
    let shared_state = State::from(config).to_shared_state();

    // Launch the Rocket server in a separate task
    tokio::spawn({
        let shared_state = shared_state.clone();
        async move { rocket(shared_state).await.launch().await }
    });

    // Start the interactive console
    start_console(&shared_state).await;

    Ok(())
}
