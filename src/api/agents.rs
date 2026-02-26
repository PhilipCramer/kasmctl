use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::agent::Agent;

use super::KasmClient;

/// Request body for updating a docker agent.
/// Only `agent_id` is required; all other fields are optional
/// and only sent when set (via `#[serde(skip_serializing_if)]`).
#[derive(Serialize)]
pub struct UpdateAgentRequest {
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores_override: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_override: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpus_override: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_prune_images: Option<String>,
}

impl KasmClient {
    pub fn get_agents(&self) -> Result<Vec<Agent>> {
        #[derive(Serialize)]
        struct Req {}

        #[derive(Deserialize)]
        struct Resp {
            agents: Vec<Agent>,
        }

        let resp: Resp = self.post("admin/get_agents", &Req {})?;
        Ok(resp.agents)
    }

    pub fn update_agent(&self, req: &UpdateAgentRequest) -> Result<Agent> {
        #[derive(Serialize)]
        struct Req<'a> {
            target_agent: &'a UpdateAgentRequest,
        }

        #[derive(Deserialize)]
        struct Resp {
            agent: Agent,
        }

        let resp: Resp = self.post("admin/update_agent", &Req { target_agent: req })?;
        Ok(resp.agent)
    }
}
