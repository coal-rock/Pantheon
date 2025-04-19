use reqwest::Url;
use std::collections::VecDeque;
use talaria::protocol::*;

use crate::{agent::AgentContext, network::Network};
use anyhow::Result;

pub struct State {
    network: Network,
    agent: AgentContext,
    instructions: VecDeque<(Option<u32>, AgentInstructionBody)>,
    responses: VecDeque<(Option<u32>, AgentResponseBody)>,
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

    pub fn get_agent_id(&self) -> u64 {
        self.agent.agent_id
    }

    pub fn set_ping(&mut self, ping: u32) {
        self.agent.ping = Some(ping);
    }

    pub fn get_ping(&self) -> Option<u32> {
        self.agent.ping
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

    pub fn get_pending_response(&mut self) -> Option<(Option<u32>, AgentResponseBody)> {
        self.responses.pop_front()
    }

    pub fn get_pending_instruction(&mut self) -> Option<(Option<u32>, AgentInstructionBody)> {
        self.instructions.pop_front()
    }

    pub fn push_instruction(&mut self, instruction: AgentInstructionBody, packet_id: Option<u32>) {
        self.instructions.push_back((packet_id, instruction));
    }

    pub fn push_response(&mut self, response: AgentResponseBody, packet_id: Option<u32>) {
        self.responses.push_back((packet_id, response));
    }

    pub fn gen_response(
        &mut self,
        response_body: AgentResponseBody,
        packet_id: Option<u32>,
    ) -> AgentResponse {
        self.network
            .gen_response(response_body, &mut self.agent, packet_id)
    }

    /// Serializes and sends a response.
    /// Returns `Result<(PingMicroseconds, AgentInstruction)>`
    pub async fn send_response(&self, response: AgentResponse) -> Result<(u32, AgentInstruction)> {
        self.network.send_response(response).await
    }
}
