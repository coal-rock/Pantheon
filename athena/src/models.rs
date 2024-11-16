use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentStatus {
    pub id: String,
    pub status: String,
    pub last_seen: String,
}
