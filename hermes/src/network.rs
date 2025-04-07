use anyhow::Result;
use local_ip_address::local_ip;
use rand::rngs::{OsRng, StdRng};
use rand::{Rng, SeedableRng, TryRngCore};
use reqwest::{Client, Url};
use talaria::helper::current_time;
use talaria::protocol::*;

use crate::agent::AgentContext;

pub struct Network {
    url: Url,
    rand: StdRng,
    used_packet_ids: Vec<u32>,
    http_client: Client,
}

impl Network {
    // We loop here to prevent collisions,
    // it's incredibly unlikely, but 10k ids is 0.04mb so it doesn't quite matter
    /// Generate unique IDs for Packets, Commmands, and other structs
    fn gen_packet_id(&mut self) -> u32 {
        loop {
            let id = self.rand.random::<u32>();

            if !self.used_packet_ids.contains(&id) {
                self.used_packet_ids.push(id);
                return id;
            }
        }
    }

    fn gen_packet_header(&mut self, agent: &AgentContext) -> PacketHeader {
        let internal_ip = match local_ip() {
            Ok(ip) => ip.to_string(),
            Err(_) => "?".to_string(),
        };

        PacketHeader {
            agent_id: agent.agent_id,
            timestamp: current_time(),
            packet_id: self.gen_packet_id(),
            polling_interval_ms: agent.polling_interval_millis,
            internal_ip,
            os: agent.os.clone(),
        }
    }

    pub fn gen_response(
        &mut self,
        response_body: AgentResponseBody,
        agent: &AgentContext,
    ) -> AgentResponse {
        let header = self.gen_packet_header(agent);

        AgentResponse {
            packet_header: header,
            packet_body: response_body,
        }
    }

    pub async fn send_response(&self, response: AgentResponse) -> Result<AgentInstruction> {
        let net_response = self
            .http_client
            .post(self.url.join("agent/monolith")?)
            .body(AgentResponse::serialize(&response)?)
            .send()
            .await?;

        let bytes = net_response.bytes().await?;
        let instruction = AgentInstruction::deserialize(&bytes.to_vec());

        Ok(instruction?)
    }

    pub fn new(url: Url) -> Network {
        let mut seed = [0u8; 32];
        OsRng.try_fill_bytes(&mut seed).unwrap();

        Network {
            url,
            rand: rand::rngs::StdRng::from_seed(seed),
            used_packet_ids: vec![].into(),
            http_client: Client::new(),
        }
    }
}
