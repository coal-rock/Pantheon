use reqwest::Url;
use std::collections::VecDeque;
use talaria::protocol::*;

use crate::{agent::AgentContext, network::Network};
use anyhow::Result;

pub struct State {
    network: Network,
    agent: AgentContext,
    instructions: VecDeque<AgentInstructionBody>,
    responses: VecDeque<AgentResponseBody>,
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

    pub fn get_polling_interval(&self) -> u64 {
        self.agent.polling_interval_millis
    }

    pub fn has_pending_responses(&self) -> bool {
        !self.responses.is_empty()
    }

    pub fn has_pending_instructions(&self) -> bool {
        !self.instructions.is_empty()
    }

    pub fn get_pending_response(&mut self) -> Option<AgentResponseBody> {
        self.responses.pop_front()
    }

    pub fn get_pending_instruction(&mut self) -> Option<AgentInstructionBody> {
        self.instructions.pop_front()
    }

    pub fn push_instruction(&mut self, instruction: AgentInstructionBody) {
        self.instructions.push_back(instruction);
    }

    pub fn push_response(&mut self, response_body: AgentResponseBody) {
        self.responses.push_back(response_body);
    }

    pub fn gen_response(&mut self, response_body: AgentResponseBody) -> AgentResponse {
        self.network.gen_response(response_body, &mut self.agent)
    }

    pub async fn send_response(&self, response: AgentResponse) -> Result<AgentInstruction> {
        self.network.send_response(response).await
    }
}
