use dioxus::prelude::*;

use anyhow::Result;
use reqwest::{Client, Url};
use serde::Deserialize;
use talaria::api::*;

#[derive(Clone)]
pub struct Api {
    api_base: Url,
    client: Client,
}

impl Api {
    pub fn new(api_base: &str) -> Api {
        Api {
            api_base: Url::parse(api_base).unwrap(),
            client: Client::new(),
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
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    pub async fn list_agents(&self) -> Result<Vec<AgentInfo>> {
        Ok(self.get("/list_agents", vec![]).await?)
    }

    pub async fn get_tartarus_info(&self) -> Result<TartarusInfo> {
        Ok(self.get("/tartarus_info", vec![]).await?)
    }
}
