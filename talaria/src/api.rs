use crate::{helper::current_time, protocol::*};
use bytesize::ByteSize;
use serde::{Deserialize, Serialize};

use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    net::SocketAddr,
    usize,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkHistoryEntry {
    instruction: AgentInstruction,
    response: Option<AgentResponse>,
}

/// Layer of abstraction over NetworkHistory.
///
/// Maps timestamp -> ID and then ID -> Entry,
/// meaning we get O(1) lookups basically for free.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkHistoryStore {
    by_id: HashMap<u32, NetworkHistoryEntry>,
    by_timestamp: BTreeSet<(u128, u32)>,
    capacity: usize,
}

impl NetworkHistoryStore {
    pub fn new(capacity: usize) -> NetworkHistoryStore {
        NetworkHistoryStore {
            by_id: HashMap::new(),
            by_timestamp: BTreeSet::new(),
            capacity,
        }
    }

    /// Inserts or overwrite entry
    pub fn insert(&mut self, entry: NetworkHistoryEntry) {
        let packet_id = match entry.instruction.header.packet_id {
            Some(packet_id) => packet_id,
            None => return,
        };

        let timestamp = entry.instruction.header.timestamp;

        self.by_id.insert(packet_id, entry);
        self.by_timestamp.insert((timestamp, packet_id));

        // trim if we are over capacity, and capacity > 0 (never trim if capacity is 0)
        if self.by_id.len() > self.capacity && self.capacity > 0 {
            if let Some(&(oldest_ts, oldest_id)) = self.by_timestamp.iter().next() {
                self.by_id.remove(&oldest_id);
                self.by_timestamp.remove(&(oldest_ts, oldest_id));
            }
        }
    }

    /// O(1)
    pub fn get(&self, packet_id: u32) -> Option<&NetworkHistoryEntry> {
        self.by_id.get(&packet_id)
    }

    /// Retrieves all hitherto entries in order of
    /// the instruction timestamp
    pub fn get_all(&self, depth: usize) -> Vec<&NetworkHistoryEntry> {
        self.by_timestamp
            .iter()
            .filter_map(|&(_, packet_id)| self.by_id.get(&packet_id))
            .take(depth)
            .collect()
    }

    /// Creates new entry containing AgentInstruction
    pub fn push_instruction(&mut self, instruction: AgentInstruction) {
        self.insert(NetworkHistoryEntry {
            instruction,
            response: None,
        })
    }

    /// Adds response to existing entry containing an instruction
    pub fn push_response(&mut self, response: AgentResponse) {
        // returns early if response contains no ID
        let entry = match response.header.packet_id {
            Some(packet_id) => self.get(packet_id),
            None => return,
        };

        // returns early if NetworkHistory does not contain matching ID
        // we should never be here(?)
        let entry = match entry {
            Some(entry) => entry,
            None => return,
        };

        self.insert(NetworkHistoryEntry {
            instruction: entry.instruction.clone(),
            response: Some(response),
        });
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Agent {
    pub nickname: Option<String>,
    pub id: u64,
    pub os: OS,
    pub external_ip: SocketAddr,
    // TODO: this maybe shouldn't be a String?
    pub internal_ip: String,
    /// Timestamp of last packet sent from agent (in ms, client time)
    pub last_packet_send: u128,
    /// Timestamp of when last packet from agent was received (in ms, server time)
    pub last_packet_recv: u128,
    /// RTT latency measured in microseconds, will be None for first packet exchange
    pub ping: Option<u32>,
    pub polling_interval_ms: u64,
    pub network_history: NetworkHistoryStore,
    pub instruction_queue: VecDeque<AgentInstructionBody>,
}

impl Agent {
    pub fn from_response(
        response: AgentResponse,
        external_ip: SocketAddr,
        history_len: usize,
    ) -> Agent {
        Agent {
            nickname: None,
            id: response.header.agent_id,
            os: response.header.os,
            external_ip,
            internal_ip: response.header.internal_ip,
            last_packet_send: response.header.timestamp,
            last_packet_recv: current_time(),
            polling_interval_ms: response.header.polling_interval_ms,
            network_history: NetworkHistoryStore::new(history_len),
            instruction_queue: vec![].into(),
            ping: None,
        }
    }

    pub fn queue_instruction(&mut self, instruction: &AgentInstructionBody) {
        self.instruction_queue.push_back(instruction.clone());
    }

    pub fn pop_instruction(&mut self) -> Option<AgentInstructionBody> {
        self.instruction_queue.pop_front()
    }

    pub fn is_active(&self) -> bool {
        // TODO: make the timeout count for inactivity be configurable
        // Currently timeout count is set to 3
        (current_time() - self.last_packet_recv) < (self.polling_interval_ms * 3).into()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AgentInfo {
    pub name: Option<String>,
    pub os: OS,
    pub id: u64,
    pub external_ip: String,
    pub internal_ip: String,
    pub status: bool,
    /// Ping measured in milliseconds
    pub ping: Option<f32>,
}

impl AgentInfo {
    pub fn header() -> String {
        fn rpad(input: &str, width: i32) -> String {
            let pad_count = std::cmp::max(width - input.len() as i32, 0) as usize;
            format!("{}{}", input, " ".repeat(pad_count))
        }

        format!(
            "{}{}{}{}{}{}{}",
            rpad("Name", 16),
            rpad("ID", 21),
            rpad("OS", 9),
            rpad("Internal IP", 20),
            rpad("External IP", 20),
            rpad("Ping", 8),
            rpad("Status", 6),
        )
    }
}

impl ToString for AgentInfo {
    fn to_string(&self) -> String {
        fn rpad(input: &str, width: i32) -> String {
            let pad_count = std::cmp::max(width - input.len() as i32, 0) as usize;
            format!("{}{}", input, " ".repeat(pad_count))
        }

        let name = self.name.clone().unwrap_or("?".into());
        let id = self.id.to_string();
        let os = self.os.os_type.to_string();
        let ping = format!("{}ms", self.ping.unwrap_or(0.0));
        let status = match self.status {
            true => "Online",
            false => "Offline",
        };

        format!(
            "{}{}{}{}{}{}{}",
            rpad(&name, 16),
            rpad(&id, 21),
            rpad(&os, 9),
            rpad(&self.internal_ip, 20),
            rpad(&self.external_ip, 20),
            rpad(&ping, 8),
            rpad(&status, 6),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// All values regarding memory/storage are
/// stored as number of bytes
pub struct TartarusInfo {
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub storage_total: u64,
    pub storage_used: u64,
    pub cpu_name: String,
    pub core_count: u64,
    pub os: String,
    pub kernel: String,
    pub hostname: String,
    pub uptime: u64,
}

impl ToString for TartarusInfo {
    fn to_string(&self) -> String {
        format!(
            r#"CPU Usage:     {:.2}%
CPU(s):        {}

Memory Usage:  {}
Storage Usage: {}

OS:            {} 
Kernel:        {}
Hostname:      {}"#,
            self.cpu_usage,
            self.cpu_name,
            format!(
                "{} / {} [{:.2}%]",
                ByteSize::b(self.memory_used).display().si(),
                ByteSize::b(self.memory_total).display().si(),
                (self.memory_used as f32 / self.memory_total as f32) * 100.0
            ),
            format!(
                "{} / {} [{:.2}%]",
                ByteSize::b(self.storage_used).display().si(),
                ByteSize::b(self.storage_total).display().si(),
                (self.storage_used as f32 / self.storage_total as f32) * 100.0
            ),
            self.os,
            self.kernel,
            self.hostname,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TartarusStats {
    pub registered_agents: u64,
    pub active_agents: u64,
    pub packets_sent: u64,
    pub packets_recv: u64,
    pub average_response_latency: f32,
    pub total_traffic: u64,
    pub windows_agents: u64,
    pub linux_agents: u64,
}

impl ToString for TartarusStats {
    fn to_string(&self) -> String {
        format!(
            r#"Registered Agents:         {}
Active Agents:             {}

Packets Sent:              {}
Packets Received:          {}

Average Response Latency:  {:.2}ms 
Total Traffic:             {}

Windows Agents:            {}
Linux Agents:              {}"#,
            self.registered_agents,
            self.active_agents,
            self.packets_sent,
            self.packets_recv,
            self.average_response_latency,
            ByteSize::b(self.total_traffic).display().si(),
            self.windows_agents,
            self.linux_agents
        )
    }
}
