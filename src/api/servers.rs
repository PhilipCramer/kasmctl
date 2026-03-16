use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::server::Server;

use super::KasmClient;

/// Parameters for creating a new server.
#[derive(Serialize)]
pub struct CreateServerParams {
    pub friendly_name: String,
    pub hostname: String,
    pub connection_type: String,
    pub connection_port: i32,
    pub zone_id: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_simultaneous_sessions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_simultaneous_users: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
}

/// Request body for updating a server.
/// Only `server_id` is required; all other fields are optional
/// and only sent when set (via `#[serde(skip_serializing_if)]`).
#[derive(Serialize)]
pub struct UpdateServerRequest {
    pub server_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_simultaneous_sessions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_simultaneous_users: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
}

impl KasmClient {
    pub fn get_servers(&self) -> Result<Vec<Server>> {
        #[derive(Serialize)]
        struct Req {}

        #[derive(Deserialize)]
        struct Resp {
            servers: Vec<Server>,
        }

        let resp: Resp = self.post("admin/get_servers", &Req {})?;
        Ok(resp.servers)
    }

    pub fn create_server(&self, params: &CreateServerParams) -> Result<Server> {
        #[derive(Serialize)]
        struct Req<'a> {
            target_server: &'a CreateServerParams,
        }

        #[derive(Deserialize)]
        struct Resp {
            server: Server,
        }

        let resp: Resp = self.post(
            "admin/create_server",
            &Req {
                target_server: params,
            },
        )?;
        Ok(resp.server)
    }

    pub fn update_server(&self, req: &UpdateServerRequest) -> Result<Server> {
        #[derive(Serialize)]
        struct Req<'a> {
            target_server: &'a UpdateServerRequest,
        }

        #[derive(Deserialize)]
        struct Resp {
            server: Server,
        }

        let resp: Resp = self.post("admin/update_server", &Req { target_server: req })?;
        Ok(resp.server)
    }

    /// Resolve an identifier to a [`Server`].
    ///
    /// Match priority:
    /// 1. Exact `server_id` match
    /// 2. `server_id` prefix match (error if ambiguous)
    /// 3. Case-insensitive `friendly_name` match (error if ambiguous)
    /// 4. Case-insensitive `hostname` match (error if ambiguous)
    ///
    /// Returns a descriptive error when no match is found or the match is ambiguous.
    pub fn resolve_server(&self, identifier: &str) -> Result<Server> {
        let servers = self.get_servers()?;

        // 1. Exact server_id match
        if let Some(s) = servers.iter().find(|s| s.server_id == identifier) {
            return Ok(s.clone());
        }

        // 2. server_id prefix match
        let prefix_matches: Vec<&Server> = servers
            .iter()
            .filter(|s| s.server_id.starts_with(identifier))
            .collect();

        match prefix_matches.len() {
            1 => return Ok(prefix_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = prefix_matches
                    .iter()
                    .map(|s| s.server_id.as_str())
                    .collect();
                anyhow::bail!(
                    "ambiguous server prefix {:?}: matches {} servers ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => {}
        }

        // 3. Case-insensitive friendly_name match
        let ident_lower = identifier.to_lowercase();
        let name_matches: Vec<&Server> = servers
            .iter()
            .filter(|s| {
                s.friendly_name
                    .as_deref()
                    .map(|n| n.to_lowercase() == ident_lower)
                    .unwrap_or(false)
            })
            .collect();

        match name_matches.len() {
            1 => return Ok(name_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = name_matches.iter().map(|s| s.server_id.as_str()).collect();
                anyhow::bail!(
                    "ambiguous server name {:?}: matches {} servers ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => {}
        }

        // 4. Case-insensitive hostname match
        let host_matches: Vec<&Server> = servers
            .iter()
            .filter(|s| {
                s.hostname
                    .as_deref()
                    .map(|h| h.to_lowercase() == ident_lower)
                    .unwrap_or(false)
            })
            .collect();

        match host_matches.len() {
            1 => Ok(host_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = host_matches.iter().map(|s| s.server_id.as_str()).collect();
                anyhow::bail!(
                    "ambiguous server hostname {:?}: matches {} servers ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => anyhow::bail!(
                "server {:?} not found (tried exact ID, ID prefix, friendly name, and hostname match)",
                identifier
            ),
        }
    }

    pub fn delete_server(&self, server_id: &str) -> Result<()> {
        #[derive(Serialize)]
        struct TargetServer<'a> {
            server_id: &'a str,
        }

        #[derive(Serialize)]
        struct Req<'a> {
            target_server: TargetServer<'a>,
        }

        #[derive(Deserialize)]
        struct Resp {}

        let _: Resp = self.post(
            "admin/delete_server",
            &Req {
                target_server: TargetServer { server_id },
            },
        )?;
        Ok(())
    }
}
