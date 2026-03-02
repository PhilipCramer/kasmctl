use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::output::display::short_id;
use crate::resource::Resource;

/// Raw scalar report data returned by `get_report`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScalarReportData {
    pub data: Option<Value>,
}

impl ScalarReportData {
    /// Extract the inner value as a u64, if present and numeric.
    pub fn as_u64(&self) -> Option<u64> {
        self.data.as_ref()?.as_u64()
    }
}

/// Per-agent resource utilization returned by `get_agent_report`.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AgentResourceReport {
    pub name: String,
    #[serde(default)]
    pub server_id: Option<String>,
    #[serde(default)]
    pub health: Option<String>,
    #[serde(default)]
    pub kasms: Option<u64>,
    #[serde(default)]
    pub disk_space: Option<u64>,
    #[serde(default)]
    pub disk_space_used: Option<u64>,
    #[serde(default)]
    pub disk_space_free: Option<u64>,
    #[serde(default)]
    pub memory_total: Option<u64>,
    #[serde(default)]
    pub memory_used: Option<u64>,
    #[serde(default)]
    pub memory_free: Option<u64>,
}

impl Resource for AgentResourceReport {
    fn resource_name() -> &'static str {
        "AgentResourceReport"
    }

    fn table_headers() -> Vec<&'static str> {
        vec!["NAME", "HEALTH", "KASMS", "MEMORY", "DISK"]
    }

    fn table_row(&self) -> Vec<String> {
        let memory = match (self.memory_used, self.memory_total) {
            (Some(used), Some(total)) => {
                format!("{}/{}", format_bytes_human(used), format_bytes_human(total))
            }
            _ => String::new(),
        };

        let disk = match (self.disk_space_used, self.disk_space) {
            (Some(used), Some(total)) => {
                format!("{}/{}", format_bytes_human(used), format_bytes_human(total))
            }
            _ => String::new(),
        };

        vec![
            short_id(&self.name).to_string(),
            self.health.clone().unwrap_or_default(),
            self.kasms.map(|v| v.to_string()).unwrap_or_default(),
            memory,
            disk,
        ]
    }

    fn table_detail(&self) -> Vec<(&'static str, String)> {
        let memory = match (self.memory_used, self.memory_total) {
            (Some(used), Some(total)) => {
                format!("{}/{}", format_bytes_human(used), format_bytes_human(total))
            }
            _ => String::new(),
        };

        let disk = match (self.disk_space_used, self.disk_space) {
            (Some(used), Some(total)) => {
                format!("{}/{}", format_bytes_human(used), format_bytes_human(total))
            }
            _ => String::new(),
        };

        vec![
            ("NAME", self.name.clone()),
            ("SERVER ID", self.server_id.clone().unwrap_or_default()),
            ("HEALTH", self.health.clone().unwrap_or_default()),
            (
                "KASMS",
                self.kasms.map(|v| v.to_string()).unwrap_or_default(),
            ),
            ("MEMORY", memory),
            ("DISK", disk),
            (
                "DISK FREE",
                self.disk_space_free
                    .map(format_bytes_human)
                    .unwrap_or_default(),
            ),
            (
                "MEMORY FREE",
                self.memory_free.map(format_bytes_human).unwrap_or_default(),
            ),
        ]
    }
}

/// Health status for a single server/component.
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub server: String,
    pub context: String,
    pub status: String,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub latency_ms: Option<u64>,
    #[serde(default)]
    pub sessions: Option<u64>,
}

/// Aggregate overview used for JSON/YAML output of the `top` command.
#[derive(Debug, Serialize, Deserialize)]
pub struct TopOverview {
    pub sessions: u64,
    pub users: u64,
    pub errors: u64,
    pub agents: Vec<AgentResourceReport>,
}

/// Format bytes as a human-readable string with one decimal place (e.g. "5.2GB").
pub fn format_bytes_human(bytes: u64) -> String {
    const GB: f64 = 1_073_741_824.0;
    const MB: f64 = 1_048_576.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.1}GB", b / GB)
    } else if b >= MB {
        format!("{:.1}MB", b / MB)
    } else {
        format!("{:.1}KB", b / 1024.0)
    }
}
