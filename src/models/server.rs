use serde::{Deserialize, Serialize};

use crate::output::display::short_id;
use crate::resource::Resource;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub server_id: String,
    #[serde(default)]
    pub friendly_name: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub connection_type: Option<String>,
    #[serde(default)]
    pub connection_port: Option<i32>,
    #[serde(default)]
    pub connection_username: Option<String>,
    #[serde(default)]
    pub connection_info: Option<String>,
    #[serde(default)]
    pub max_simultaneous_sessions: Option<i32>,
    #[serde(default)]
    pub max_simultaneous_users: Option<i32>,
    #[serde(default)]
    pub zone_id: Option<String>,
    #[serde(default)]
    pub pool_id: Option<String>,
}

impl Resource for Server {
    fn resource_name() -> &'static str {
        "Server"
    }

    fn table_headers() -> Vec<&'static str> {
        vec![
            "SERVER ID",
            "NAME",
            "HOSTNAME",
            "TYPE",
            "ENABLED",
            "SESSIONS",
        ]
    }

    fn table_row(&self) -> Vec<String> {
        vec![
            short_id(&self.server_id).to_string(),
            self.friendly_name.clone().unwrap_or_default(),
            self.hostname.clone().unwrap_or_default(),
            self.connection_type.clone().unwrap_or_default(),
            self.enabled.map(|v| v.to_string()).unwrap_or_default(),
            self.max_simultaneous_sessions
                .map(|v| v.to_string())
                .unwrap_or_default(),
        ]
    }

    fn table_detail(&self) -> Vec<(&'static str, String)> {
        vec![
            ("SERVER ID", self.server_id.clone()),
            (
                "FRIENDLY NAME",
                self.friendly_name.clone().unwrap_or_default(),
            ),
            ("HOSTNAME", self.hostname.clone().unwrap_or_default()),
            (
                "ENABLED",
                self.enabled.map(|v| v.to_string()).unwrap_or_default(),
            ),
            (
                "CONNECTION TYPE",
                self.connection_type.clone().unwrap_or_default(),
            ),
            (
                "CONNECTION PORT",
                self.connection_port
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            (
                "CONNECTION USERNAME",
                self.connection_username.clone().unwrap_or_default(),
            ),
            (
                "CONNECTION INFO",
                self.connection_info.clone().unwrap_or_default(),
            ),
            (
                "MAX SIMULTANEOUS SESSIONS",
                self.max_simultaneous_sessions
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            (
                "MAX SIMULTANEOUS USERS",
                self.max_simultaneous_users
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            ("ZONE ID", self.zone_id.clone().unwrap_or_default()),
            ("POOL ID", self.pool_id.clone().unwrap_or_default()),
        ]
    }
}
