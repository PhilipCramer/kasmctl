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
