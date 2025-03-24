use dioxus::prelude::*;

use anyhow::Result;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use talaria::api::*;
use talaria::console::*;

#[derive(Clone)]
pub struct Api {
    api_base: Url,
    client: Client,
    token: String,
}

impl Api {
    pub fn new(api_base: &str) -> Api {
        Api {
            api_base: Url::parse(api_base).unwrap(),
            client: Client::new(),
            token: String::from("bb123#123"),
        }
    }

    fn make_api_path(&self, endpoint: &str) -> Result<Url> {
        Ok(self.api_base.join(&format!("/admin{}", endpoint))?)
    }

    async fn get<T>(&self, endpoint: &str, query_params: Vec<(&str, &str)>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(self
            .client
            .get(self.make_api_path(endpoint)?)
            .query(&query_params)
            .header("Authorization", &self.token)
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    async fn post<O, I>(&self, endpoint: &str, data: I) -> Result<O>
    where
        I: Serialize,
        O: for<'de> Deserialize<'de>,
    {
        Ok(self
            .client
            .post(self.make_api_path(endpoint)?)
            .json(&data)
            .header("Authorization", &self.token)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn list_agents(&self) -> Result<Vec<AgentInfo>> {
        Ok(self.get("/list_agents", vec![]).await?)
    }

    pub async fn get_tartarus_info(&self) -> Result<TartarusInfo> {
        Ok(self.get("/tartarus_info", vec![]).await?)
    }

    pub async fn get_tartarus_stats(&self) -> Result<TartarusStats> {
        Ok(self.get("/tartarus_stats", vec![]).await?)
    }

    pub async fn console(&self, command_context: CommandContext) -> Result<ConsoleResponse> {
        Ok(self.post("/console/monolith", command_context).await?)
    }
}
