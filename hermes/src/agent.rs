// use local_ip_address::local_ip;
// use std::process::Command;
// use sys_info::{hostname, os_type};

// pub async fn get_system_info() -> Result<SystemInfo, Box<dyn std::error::Error>> {
//     let os = os_type()?;
//     let ip = local_ip()?.to_string();
//     Ok(SystemInfo { os, ip })
// }
//
// pub fn execute_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let output = Command::new("sh").arg("-c").arg(command).output()?;
//
//     Ok(String::from_utf8_lossy(&output.stdout).to_string())
// }

use mac_address::get_mac_address;
use rand::Rng;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::time::SystemTime;
use sys_info;
use talaria::{AgentInstruction, AgentResponse, PacketHeader};

pub struct Agent {
    pub server_addr: String,
    pub agent_id: u64,
    pub polling_interval_millis: u64,
    pub rec_log: Vec<Result<AgentInstruction, reqwest::Error>>,
    pub send_log: Vec<AgentResponse>,
    pub http_client: Client,
    rand: rand::rngs::ThreadRng,
    used_ids: Vec<u32>,
}

impl Agent {
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
            timestamp: Agent::get_timestamp(),
            packet_id: self.gen_id(),
        }
    }

    // We loop here to prevent collisions,
    // it's incredibly unlikely, but 10k ids 0.04mb so it doesn't quite matter
    pub fn gen_id(&mut self) -> u32 {
        loop {
            let id = self.rand.gen::<u32>();

            if !self.used_ids.contains(&id) {
                self.used_ids.push(id);
                return id;
            }
        }
    }

    pub fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn new(server_addr: String) -> Agent {
        Agent {
            server_addr,
            agent_id: Agent::generate_deterministic_uuid(),
            polling_interval_millis: 10000,
            send_log: vec![],
            rec_log: vec![],
            http_client: Client::new(),
            rand: rand::thread_rng(),
            used_ids: vec![],
        }
    }
}
