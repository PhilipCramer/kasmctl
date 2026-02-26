use serde::{Deserialize, Serialize};

use crate::output::display::short_id;
use crate::resource::Resource;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    pub agent_id: String,
    #[serde(default)]
    pub server_id: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub zone_id: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub cores: Option<f64>,
    #[serde(default)]
    pub memory: Option<i64>,
    #[serde(default)]
    pub gpus: Option<f64>,
    #[serde(default)]
    pub cores_override: Option<f64>,
    #[serde(default)]
    pub memory_override: Option<i64>,
    #[serde(default)]
    pub gpus_override: Option<f64>,
    #[serde(default)]
    pub auto_prune_images: Option<String>,
}

impl Resource for Agent {
    fn resource_name() -> &'static str {
        "Agent"
    }

    fn table_headers() -> Vec<&'static str> {
        vec![
            "AGENT ID", "HOSTNAME", "STATUS", "ENABLED", "CORES", "MEMORY",
        ]
    }

    fn table_row(&self) -> Vec<String> {
        vec![
            short_id(&self.agent_id).to_string(),
            self.hostname.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.enabled.map(|v| v.to_string()).unwrap_or_default(),
            self.cores.map(|v| v.to_string()).unwrap_or_default(),
            self.memory.map(format_bytes).unwrap_or_default(),
        ]
    }

    fn table_detail(&self) -> Vec<(&'static str, String)> {
        vec![
            ("AGENT ID", self.agent_id.clone()),
            ("SERVER ID", self.server_id.clone().unwrap_or_default()),
            ("HOSTNAME", self.hostname.clone().unwrap_or_default()),
            ("STATUS", self.status.clone().unwrap_or_default()),
            ("ZONE ID", self.zone_id.clone().unwrap_or_default()),
            (
                "ENABLED",
                self.enabled.map(|v| v.to_string()).unwrap_or_default(),
            ),
            (
                "CORES",
                self.cores.map(|v| v.to_string()).unwrap_or_default(),
            ),
            ("MEMORY", self.memory.map(format_bytes).unwrap_or_default()),
            ("GPUS", self.gpus.map(|v| v.to_string()).unwrap_or_default()),
            (
                "CORES OVERRIDE",
                self.cores_override
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            (
                "MEMORY OVERRIDE",
                self.memory_override.map(format_bytes).unwrap_or_default(),
            ),
            (
                "GPUS OVERRIDE",
                self.gpus_override
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            (
                "AUTO PRUNE IMAGES",
                self.auto_prune_images.clone().unwrap_or_default(),
            ),
        ]
    }
}

fn format_bytes(bytes: i64) -> String {
    if bytes < 0 {
        return bytes.to_string();
    }
    const GB: i64 = 1_073_741_824;
    const MB: i64 = 1_048_576;
    if bytes >= GB && bytes % GB == 0 {
        format!("{}GB", bytes / GB)
    } else if bytes >= MB && bytes % MB == 0 {
        format!("{}MB", bytes / MB)
    } else {
        bytes.to_string()
    }
}
