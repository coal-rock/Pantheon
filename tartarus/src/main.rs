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
mod state;
mod statistics;

use crate::console_interface::start_console;
use config::Config;
use state::{SharedState, State};

use cors::CORS;
use rocket::{Build, Rocket};
use talaria::scripting::Script;

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

    println!("\nConfiguration:");
    println!("--------------------------");
    print!("{}", toml::to_string_pretty(&config.clone()).unwrap());
    println!("--------------------------\n");

    let shared_state = State::from(config).to_shared_state();

    let script = Script::from_str(
        r#"---
    name = "script" 
    description = "this is a script" 
    
    [[params]]
    name = "param 1"
    arg_name = "param_1"
    description = "this is a string param"
    type = "string"
    placeholder = "example_string"
---

    "#,
    );

    println!("{:#?}", script);

    shared_state.write().await.add_script(script.unwrap());

    let rocket = tokio::spawn(rocket(shared_state.clone()).await.launch());
    let console = tokio::spawn(start_console(shared_state.clone()));

    let _ = tokio::join!(rocket, console);

    Ok(())
}
