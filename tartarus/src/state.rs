use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use talaria::api::*;
use talaria::console::*;
use talaria::protocol::*;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::statistics::Statistics;

#[derive(Default, Clone)]
pub struct State {
    pub config: Config,
    pub statistics: Statistics,
    pub agents: HashMap<u128, Agent>,
    pub groups: HashMap<String, Vec<u128>>,
    // TODO: This being sequential isn't the best idea, but it works for now
    curr_packet_id: u32,
}

impl State {
    pub fn get_agent(&self, agent_id: &u128) -> Option<&Agent> {
        self.agents.get(&agent_id)
    }

    // this shouldn't be public because we can't trust the caller to not mess up state
    fn get_agent_mut(&mut self, agent_id: &u128) -> Option<&mut Agent> {
        self.agents.get_mut(&agent_id)
    }

    pub fn gen_packet_id(&mut self) -> u32 {
        self.curr_packet_id += 1;
        self.curr_packet_id - 1
    }

    /// Attempts to register an agent
    ///
    /// Returns `true` if agent was not previously present
    /// Returns `false` if agent already exists
    pub fn try_register_agent(&mut self, response: &AgentResponse, ext_ip: &SocketAddr) -> bool {
        let agent_id = response.header.agent_id;

        match self.get_agent(&agent_id) {
            Some(_) => return false,
            None => {}
        };

        let agent = Agent::from_response(response.clone(), *ext_ip);
        self.agents.insert(agent_id, agent);

        true
    }

    /// Attempts to get any pending instructions
    ///
    /// returns `None` if agent isn't found, or if queue is empty
    pub fn pop_instruction(&mut self, agent_id: &u128) -> Option<AgentInstructionBody> {
        match self.agents.get_mut(&agent_id) {
            Some(agent) => agent.pop_instruction(),
            None => None,
        }
    }

    pub fn get_network_history(
        &self,
        agent_id: &u128,
        depth: usize,
    ) -> Option<Vec<&NetworkHistoryEntry>> {
        let agent = match self.get_agent(agent_id) {
            Some(agent) => agent,
            None => return None,
        };

        Some(agent.network_history.get_all(depth))
    }

    pub fn push_instruction_to_history(&mut self, instruction: &AgentInstruction, agent_id: &u128) {
        match self.get_agent_mut(&agent_id) {
            Some(agent) => agent.network_history.push_instruction(instruction.clone()),
            None => return,
        };
    }

    pub fn push_response_to_history(&mut self, response: &AgentResponse, agent_id: &u128) {
        match self.get_agent_mut(&agent_id) {
            Some(agent) => agent.network_history.push_response(response.clone()),
            None => return,
        };
    }

    pub fn get_agent_by_ident(&self, ident: &AgentIdentifier) -> Option<&Agent> {
        match ident {
            AgentIdentifier::Nickname { nickname } => {
                for (_, agent) in &self.agents {
                    if agent.nickname == Some(nickname.clone()) {
                        return Some(&agent);
                    }
                }
            }
            AgentIdentifier::ID { id } => return self.agents.get(&id),
        }

        return None;
    }

    ///TODO:this shouldn't be pub
    pub fn get_agent_by_ident_mut(&mut self, ident: &AgentIdentifier) -> Option<&mut Agent> {
        match ident {
            AgentIdentifier::Nickname { nickname } => {
                for (id, agent) in self.agents.clone() {
                    if agent.nickname == Some(nickname.clone()) {
                        return self.agents.get_mut(&id);
                    }
                }
            }
            AgentIdentifier::ID { id } => return self.agents.get_mut(&id),
        }

        return None;
    }

    pub fn from(config: Config) -> State {
        State {
            config,
            agents: HashMap::new(),
            groups: HashMap::new(),
            statistics: Statistics::default(),
            curr_packet_id: 0,
        }
    }

    pub fn to_shared_state(&self) -> SharedState {
        Arc::new(RwLock::new(self.clone()))
    }
}

pub type SharedState = Arc<RwLock<State>>;
