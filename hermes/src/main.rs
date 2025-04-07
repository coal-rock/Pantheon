pub mod agent;
pub mod network;
pub mod state;

use state::State;
use std::sync::Arc;
use talaria::helper::*;
use talaria::protocol::*;
use tokio::sync::RwLock;
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

    let poll = tokio::spawn(poll(state.clone()));
    let eval = tokio::spawn(eval(state.clone()));

    let _ = tokio::join!(poll, eval);
}

async fn poll(state: Arc<RwLock<State>>) {
    let interval = state.read().await.get_polling_interval();
    let mut interval = time::interval(Duration::from_millis(interval));

    loop {
        interval.tick().await;

        let response_body = match state.write().await.get_pending_response() {
            Some(response) => response,
            None => AgentResponseBody::Heartbeat,
        };

        let response = state.write().await.gen_response(response_body);
        let instruction = state.read().await.send_response(response).await;

        match instruction {
            Ok(instruction) => {
                devlog!("Got instruction: {:#?}", instruction);
                state
                    .write()
                    .await
                    .push_instruction(instruction.packet_body);
            }
            Err(err) => {
                devlog!("Failed to properly communicate with server: {:#?}", err);
            }
        }
    }
}

async fn eval(state: Arc<RwLock<State>>) {
    let mut interval = time::interval(Duration::from_millis(100));

    loop {
        interval.tick().await;

        let instruction = match state.write().await.get_pending_instruction() {
            Some(instruction) => instruction,
            None => continue,
        };

        match instruction {
            _ => devlog!("Evaluating instruction: {:#?}", instruction),
        }
    }
}
