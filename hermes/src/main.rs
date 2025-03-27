pub mod agent;
pub mod harpe;
pub mod network;

use agent::AgentContext;
use std::time::Duration;
use talaria::helper::*;
use tokio::time::sleep;

const PORT: u16 = env!("PORT", "environment variable `PORT` not defined");

const HOST: &'static str = env!("HOST", "environment variable `HOST` not defined");

const POLL_INTERVAL_MS: u64 = env!(
    "POLL_INTERVAL_MS",
    "environment variable `POLL_INTERVAL_MS not defined"
)
.parse()
.expect("Invalid port");

#[tokio::main]
async fn main() {
    let mut agent = AgentContext::new(HOST, PORT, POLL_INTERVAL_MS);

    loop {
        match network::send_heartbeat(&mut agent).await {
            Ok(instruction) => {
                devlog!("Got instruction: {:#?}", instruction);
                match network::handle_response(&mut agent, instruction).await {
                    Ok(_) => devlog!("Successfully handled response"),
                    Err(err) => devlog!("Failed to handle response\n{:?}", err),
                }
            }
            Err(err) => devlog!("Failed to communicate with server\n{:?}", err),
        }

        // FIXME: whoever wrote this does not understand the nature of async code (me [cole])
        // we are waiting X amount of time from the end of the last execution,
        // meaning if we intend to call function Y every X seconds, we are actually
        // calling function Y every X + N seconds, where N is the execution time of Y
        sleep(Duration::from_millis(agent.polling_interval_millis)).await;
    }
}
