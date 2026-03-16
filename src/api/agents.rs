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

    /// Resolve an identifier to an [`Agent`].
    ///
    /// Match priority:
    /// 1. Exact `agent_id` match
    /// 2. `agent_id` prefix match (error if ambiguous)
    /// 3. Case-insensitive `hostname` match (error if ambiguous)
    ///
    /// Returns a descriptive error when no match is found or the match is ambiguous.
    pub fn resolve_agent(&self, identifier: &str) -> Result<Agent> {
        let agents = self.get_agents()?;

        // 1. Exact agent_id match
        if let Some(a) = agents.iter().find(|a| a.agent_id == identifier) {
            return Ok(a.clone());
        }

        // 2. agent_id prefix match
        let prefix_matches: Vec<&Agent> = agents
            .iter()
            .filter(|a| a.agent_id.starts_with(identifier))
            .collect();

        match prefix_matches.len() {
            1 => return Ok(prefix_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = prefix_matches.iter().map(|a| a.agent_id.as_str()).collect();
                anyhow::bail!(
                    "ambiguous agent prefix {:?}: matches {} agents ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => {}
        }

        // 3. Case-insensitive hostname match
        let ident_lower = identifier.to_lowercase();
        let name_matches: Vec<&Agent> = agents
            .iter()
            .filter(|a| {
                a.hostname
                    .as_deref()
                    .map(|h| h.to_lowercase() == ident_lower)
                    .unwrap_or(false)
            })
            .collect();

        match name_matches.len() {
            1 => Ok(name_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = name_matches.iter().map(|a| a.agent_id.as_str()).collect();
                anyhow::bail!(
                    "ambiguous agent hostname {:?}: matches {} agents ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => anyhow::bail!(
                "agent {:?} not found (tried exact ID, ID prefix, and hostname match)",
                identifier
            ),
        }
    }
}
