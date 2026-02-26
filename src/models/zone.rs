use serde::{Deserialize, Serialize};

use crate::output::display::short_id;
use crate::resource::Resource;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    pub zone_id: String,
    #[serde(default)]
    pub zone_name: Option<String>,
    #[serde(default)]
    pub allow_origin_domain: Option<String>,
    #[serde(default)]
    pub upstream_auth_address: Option<String>,
    #[serde(default)]
    pub load_balancing_strategy: Option<String>,
    #[serde(default)]
    pub search_alternate_zones: Option<bool>,
    #[serde(default)]
    pub prioritize_static_agents: Option<bool>,
    #[serde(default)]
    pub proxy_connections: Option<bool>,
    #[serde(default)]
    pub proxy_hostname: Option<String>,
    #[serde(default)]
    pub proxy_path: Option<String>,
    #[serde(default)]
    pub proxy_port: Option<i32>,
}

impl Resource for Zone {
    fn resource_name() -> &'static str {
        "Zone"
    }

    fn table_headers() -> Vec<&'static str> {
        vec!["ZONE ID", "NAME", "LOAD BALANCING", "PROXY"]
    }

    fn table_row(&self) -> Vec<String> {
        vec![
            short_id(&self.zone_id).to_string(),
            self.zone_name.clone().unwrap_or_default(),
            self.load_balancing_strategy.clone().unwrap_or_default(),
            self.proxy_connections
                .map(|v| v.to_string())
                .unwrap_or_default(),
        ]
    }

    fn table_detail(&self) -> Vec<(&'static str, String)> {
        vec![
            ("ZONE ID", self.zone_id.clone()),
            ("ZONE NAME", self.zone_name.clone().unwrap_or_default()),
            (
                "ALLOW ORIGIN DOMAIN",
                self.allow_origin_domain.clone().unwrap_or_default(),
            ),
            (
                "UPSTREAM AUTH ADDRESS",
                self.upstream_auth_address.clone().unwrap_or_default(),
            ),
            (
                "LOAD BALANCING STRATEGY",
                self.load_balancing_strategy.clone().unwrap_or_default(),
            ),
            (
                "SEARCH ALTERNATE ZONES",
                self.search_alternate_zones
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            (
                "PRIORITIZE STATIC AGENTS",
                self.prioritize_static_agents
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            (
                "PROXY CONNECTIONS",
                self.proxy_connections
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            (
                "PROXY HOSTNAME",
                self.proxy_hostname.clone().unwrap_or_default(),
            ),
            ("PROXY PATH", self.proxy_path.clone().unwrap_or_default()),
            (
                "PROXY PORT",
                self.proxy_port.map(|v| v.to_string()).unwrap_or_default(),
            ),
        ]
    }
}
