use mac_address::get_mac_address;
use rand::Rng;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::time::SystemTime;
use sys_info;
use talaria::protocol::*;

pub struct AgentContext {
    pub server_addr: String,
    pub server_port: u16,
    pub agent_id: u64,
    pub polling_interval_millis: u64,
    pub http_client: Client,
    rand: rand::rngs::ThreadRng,
    used_ids: Vec<u32>,
}

impl AgentContext {
    // Mashes together a bunch of staticish system information,
    // Takes the Sha256 hash of aformentioned data,
    // Returns the first 64 bits of the aformentioned hash
    pub fn generate_deterministic_uuid() -> u64 {
        let hostname: String = sys_info::hostname().unwrap();
        let os_version: String = sys_info::os_release().unwrap();
        let cpu_num: u32 = sys_info::cpu_num().unwrap();
        let os_type: String = sys_info::os_type().unwrap();
        let mac_address: [u8; 6] = get_mac_address().unwrap().unwrap().bytes();

        let unique_info = format!(
            "{}:{}:{}:{}:{:?}",
            hostname, os_version, cpu_num, os_type, mac_address
        );

        let mut hasher = Sha256::new();
        hasher.update(unique_info.as_bytes());
        let hash = hasher.finalize();

        u64::from_be_bytes(hash[0..8].try_into().unwrap())
    }

    pub fn generate_packet_header(&mut self) -> PacketHeader {
        PacketHeader {
            agent_id: self.agent_id,
            timestamp: AgentContext::get_timestamp(),
            packet_id: self.gen_id(),
            os: Some(sys_info::os_type().unwrap()),
        }
    }

    // We loop here to prevent collisions,
    // it's incredibly unlikely, but 10k ids 0.04mb so it doesn't quite matter
    /// Generate unique IDs for Packets, Commmands, and other structs
    pub fn gen_id(&mut self) -> u32 {
        loop {
            let id = self.rand.gen::<u32>();

            if !self.used_ids.contains(&id) {
                self.used_ids.push(id);
                return id;
            }
        }
    }

    /// Helper function, should be used if time is needed
    pub fn get_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.server_addr, self.server_port)
    }

    pub fn new(server_addr: &str, server_port: u16, polling_interval_millis: u64) -> AgentContext {
        AgentContext {
            server_addr: server_addr.to_string(),
            server_port,
            agent_id: AgentContext::generate_deterministic_uuid(),
            polling_interval_millis: 10000,
            http_client: Client::new(),
            rand: rand::thread_rng(),
            used_ids: vec![],
        }
    }
}
