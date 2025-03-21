use crate::server_fn::codec::*;
use crate::tartarus::State;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use talaria::api::Agent;

#[server(
  name = GetAgents,
  prefix = "/api",
  endpoint = "get_agents",
  input = GetUrl,
)]
async fn get_agents(depth: usize) -> Result<HashMap<u64, Agent>, ServerFnError> {
    let FromContext::<State>(state) = extract().await?;
    let agents = Arc::clone(&state.agents);
    let agents = agents.lock()?;

    Ok(agents
        .clone()
        .into_iter()
        .map(|(id, agent)| (id, agent.truncate(depth)))
        .collect())
}
