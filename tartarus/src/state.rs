use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use sysinfo::Disks;
use sysinfo::System;
use talaria::api::*;
use talaria::protocol::*;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::statistics::Statistics;

#[derive(Default, Clone)]
pub struct State {
    pub config: Config,
    pub statistics: Statistics,
    pub agents: HashMap<u64, Agent>,
    pub groups: HashMap<String, Vec<u64>>,
    scripts: HashMap<String, Script>,
    // TODO: This being sequential isn't the best idea, but it works for now
    curr_packet_id: u32,
}

impl State {
    pub fn gen_packet_id(&mut self) -> u32 {
        self.curr_packet_id += 1;
        self.curr_packet_id - 1
    }

    pub fn lookup_agent(&self, nickname: &str) -> Option<u64> {
        self.agents.iter().find_map(|(id, agent)| {
            if agent.nickname.as_deref() == Some(nickname) {
                Some(*id)
            } else {
                None
            }
        })
    }

    pub fn get_agent(&self, agent_id: &u64) -> Option<&Agent> {
        self.agents.get(&agent_id)
    }

    // this shouldn't be public because we can't trust the caller to not mess up state
    pub fn get_agent_mut(&mut self, agent_id: &u64) -> Option<&mut Agent> {
        self.agents.get_mut(&agent_id)
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

        let agent = Agent::from_response(response.clone(), *ext_ip, self.config.history_buf_len);
        self.agents.insert(agent_id, agent);

        true
    }

    /// Attempts to get any pending instructions
    ///
    /// returns `None` if agent isn't found, or if queue is empty
    pub fn pop_instruction(&mut self, agent_id: &u64) -> Option<AgentInstructionBody> {
        match self.agents.get_mut(&agent_id) {
            Some(agent) => agent.pop_instruction(),
            None => None,
        }
    }

    pub fn push_instruction(&mut self, agent_id: &u64, instruction: &AgentInstructionBody) -> bool {
        match self.agents.get_mut(&agent_id) {
            Some(agent) => {
                agent.queue_instruction(instruction);
                true
            }
            None => false,
        }
    }

    pub fn get_network_history(
        &self,
        agent_id: &u64,
        depth: usize,
    ) -> Option<Vec<&NetworkHistoryEntry>> {
        let agent = match self.get_agent(agent_id) {
            Some(agent) => agent,
            None => return None,
        };

        Some(agent.network_history.get_all(depth))
    }

    pub fn push_instruction_to_history(&mut self, instruction: &AgentInstruction, agent_id: &u64) {
        match self.get_agent_mut(&agent_id) {
            Some(agent) => agent.network_history.push_instruction(instruction.clone()),
            None => return,
        };
    }

    pub fn push_response_to_history(&mut self, response: &AgentResponse, agent_id: &u64) {
        match self.get_agent_mut(&agent_id) {
            Some(agent) => agent.network_history.push_response(response.clone()),
            None => return,
        };
    }

    pub fn get_tartarus_info(&self) -> TartarusInfo {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_name = if let Some(cpu) = sys.cpus().first() {
            Some(cpu.brand())
        } else {
            None
        };

        let mut storage_total = None;
        let mut storage_used = None;

        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            if disk.mount_point() == Path::new("/") {
                storage_total = Some(disk.total_space());
                storage_used = Some(disk.total_space() - disk.available_space())
            }
        }

        TartarusInfo {
            cpu_usage: sys.global_cpu_usage(),
            memory_total: sys.total_memory(),
            memory_used: sys.used_memory(),
            storage_total: storage_total.unwrap(),
            storage_used: storage_used.unwrap(),
            cpu_name: cpu_name.unwrap().to_string(),
            core_count: sys.cpus().len() as u64,
            os: System::long_os_version().unwrap(),
            kernel: System::kernel_version().unwrap(),
            hostname: System::host_name().unwrap(),
            uptime: System::uptime(),
        }
    }

    pub fn get_tartarus_stats(&self) -> TartarusStats {
        let agents = self.agents.clone();
        let statistics = self.statistics.clone();

        TartarusStats {
            registered_agents: agents.len() as u64,
            active_agents: agents
                .iter()
                .map(|(_id, agent)| agent.is_active() as u64)
                .sum(),
            packets_sent: statistics.packets_sent,
            packets_recv: statistics.packets_recv,
            average_response_latency: statistics.get_average_latency(),
            total_traffic: statistics.get_total_traffic(),
            windows_agents: 0, // TODO: fix
            linux_agents: agents.len() as u64,
        }
    }

    pub fn get_agent_list(&self) -> Vec<AgentInfo> {
        self.agents
            .clone()
            .iter()
            .map(|(_, a)| AgentInfo {
                status: a.is_active(),
                name: a.nickname.clone(),
                id: a.id,
                external_ip: a.external_ip.to_string(),
                internal_ip: a.internal_ip.to_string(),
                os: a.os.clone(),
                ping: a.ping.map(|p| p as f32 / 1000.0),
            })
            .collect::<Vec<AgentInfo>>()
    }

    pub fn get_agent_history(&self, agent_id: u64, count: usize) -> Vec<NetworkHistoryEntry> {
        let agents = self.agents.clone();

        match agents.get(&agent_id) {
            Some(agent) => agent
                .network_history
                .get_all(count)
                .into_iter()
                .cloned()
                .collect(),

            None => vec![],
        }
    }

    pub fn get_script(&self, script_name: String) -> Option<&Script> {
        self.scripts.get(&script_name)
    }

    pub fn get_all_scripts(&self) -> Vec<&Script> {
        self.scripts.values().collect()
    }

    pub fn from(config: Config) -> State {
        State {
            config,
            agents: HashMap::new(),
            groups: HashMap::new(),
            scripts: HashMap::new(),
            statistics: Statistics::default(),
            curr_packet_id: 0,
        }
    }

    pub fn to_shared_state(&self) -> SharedState {
        Arc::new(RwLock::new(self.clone()))
    }
}

pub type SharedState = Arc<RwLock<State>>;
