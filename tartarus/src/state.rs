use std::collections::HashMap;
use std::sync::Arc;

use talaria::api::*;
use talaria::console::*;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::statistics::Statistics;

// Shared state for active listeners
#[derive(Default, Clone)]
pub struct State {
    pub config: Config,
    pub agents: HashMap<u64, Agent>,
    pub groups: HashMap<String, Vec<u64>>,
    pub statistics: Statistics,
}

impl State {
    pub fn get_agent(&self, ident: &AgentIdentifier) -> Option<&Agent> {
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

    pub fn get_agent_mut(&mut self, ident: &AgentIdentifier) -> Option<&mut Agent> {
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
        }
    }

    pub fn to_shared_state(&self) -> SharedState {
        Arc::new(RwLock::new(self.clone()))
    }
}

// Wrap in Arc and RwLock for safe concurrent access
pub type SharedState = Arc<RwLock<State>>;
