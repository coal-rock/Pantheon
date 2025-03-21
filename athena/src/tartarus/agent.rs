use crate::tartarus::State;
use dioxus::prelude::*;
use rkyv::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use talaria::api::Agent;
use talaria::protocol::{AgentInstruction, AgentResponse};

use server_fn::codec::*;

#[server(
  name = Monolith,
  prefix = "/agent",
  endpoint = "monolith",
  input = Rkyv,
  output = Rkyv,
)]
async fn monolith(agent_response: AgentResponse) -> Result<AgentInstruction, ServerFnError> {
    Ok(String::from("hello, world"))
}
