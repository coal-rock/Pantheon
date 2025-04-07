use reqwest::Url;
use std::collections::VecDeque;
use talaria::protocol::*;

use crate::{agent::AgentContext, network::Network};
use anyhow::Result;

pub struct State {
    network: Network,
    agent: AgentContext,
    instructions: VecDeque<AgentInstruction>,
    responses: VecDeque<AgentResponse>,
}

impl State {
    pub fn new(url: &str, poll_interval_ms: u64) -> Result<State> {
        Ok(State {
            network: Network::new(Url::parse(url)?),
            instructions: vec![].into(),
            responses: vec![].into(),
            agent: AgentContext::new(poll_interval_ms)?,
        })
    }
}
