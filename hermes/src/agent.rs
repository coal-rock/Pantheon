use anyhow::{anyhow, Result};
use mac_address::get_mac_address;
use sha2::{Digest, Sha256};
use sysinfo;
use sysinfo::System;
use talaria::protocol::*;

pub struct AgentContext {
    pub agent_id: u64,
    pub ping: Option<u32>,
    pub polling_interval_millis: u64,
    pub os: OS,
}

impl AgentContext {
    // Mashes together a bunch of staticish system information,
    // Takes the Sha256 hash of aformentioned data,
    // Returns the first 64 bits of the aformentioned hash
    fn generate_deterministic_uuid() -> Result<u64> {
        // TODO: proper
        let hostname: String = System::host_name().ok_or(anyhow!("unable to get hostname"))?;
        let os_version: String = System::os_version().ok_or(anyhow!("unable to get os version"))?;
        let cpu_num = System::physical_core_count().ok_or(anyhow!("unable to get core count"))?;
        let os_type: String = System::name().ok_or(anyhow!("unable to get system name"))?;
        let mac_address: [u8; 6] = get_mac_address()?
            .ok_or(anyhow!("unable to get mac addr"))?
            .bytes();

        let unique_info = format!(
            "{}:{}:{}:{}:{:?}",
            hostname, os_version, cpu_num, os_type, mac_address
        );

        let mut hasher = Sha256::new();
        hasher.update(unique_info.as_bytes());
        let hash = hasher.finalize();

        Ok(u64::from_be_bytes(hash[0..8].try_into()?))
    }

    pub fn new(polling_interval_millis: u64) -> Result<AgentContext> {
        let os = OS::from(&std::env::consts::OS, System::long_os_version());

        Ok(AgentContext {
            agent_id: AgentContext::generate_deterministic_uuid()?,
            polling_interval_millis,
            os,
            ping: None,
        })
    }
}
