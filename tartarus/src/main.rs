#[macro_use]
extern crate rocket;
extern crate log;

mod admin;
mod agent;
mod auth;
mod config;
mod console_interface;
mod console_lib;
mod console_net;
mod cors;
mod scripting;
mod state;
mod statistics;

use crate::console_interface::start_console;
use config::Config;
use scripting::Script;
use state::{SharedState, State};

use cors::CORS;
use rocket::{Build, Rocket};

async fn rocket(shared_state: SharedState) -> Rocket<Build> {
    let config = shared_state.read().await.config.clone();

    rocket::build()
        .mount("/", cors::routes())
        .mount("/api/admin", admin::routes())
        .mount("/api/admin/console", console_net::routes())
        .mount("/api/agent", agent::routes())
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

    let config = Config::new("tartarus.toml");

    println!(
        "{:#?}",
        Script::from_str(
            r#"
            name = "Test Script"
            description = "This script is a test"

            [[params]]
            name = "param1"
            description = "param1 description"
            type = "String"
            placeholder = "hello"
            "#
        )
    );

    println!("\nConfiguration:");
    println!("--------------------------");
    print!("{}", toml::to_string_pretty(&config.clone()).unwrap());
    println!("--------------------------\n");

    let shared_state = State::from(config).to_shared_state();

    let rocket = tokio::spawn(rocket(shared_state.clone()).await.launch());
    let console = tokio::spawn(start_console(shared_state.clone()));

    let _ = tokio::join!(rocket, console);

    Ok(())
}
