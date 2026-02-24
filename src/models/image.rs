use serde::{Deserialize, Serialize};

use crate::output::display::short_id;
use crate::resource::Resource;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub image_id: String,
    #[serde(default)]
    pub friendly_name: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub cores: Option<f64>,
    #[serde(default)]
    pub memory: Option<i64>,
    #[serde(default)]
    pub image_src: Option<String>,
}

impl Resource for Image {
    fn resource_name() -> &'static str {
        "Image"
    }

    fn table_headers() -> Vec<&'static str> {
        vec![
            "IMAGE ID",
            "NAME",
            "DOCKER IMAGE",
            "ENABLED",
            "CORES",
            "MEMORY",
        ]
    }

    fn table_row(&self) -> Vec<String> {
        vec![
            short_id(&self.image_id).to_string(),
            self.friendly_name.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.enabled.map(|v| v.to_string()).unwrap_or_default(),
            self.cores.map(|v| v.to_string()).unwrap_or_default(),
            self.memory.map(format_bytes).unwrap_or_default(),
        ]
    }

    fn table_detail(&self) -> Vec<(&'static str, String)> {
        vec![
            ("IMAGE ID", self.image_id.clone()),
            (
                "FRIENDLY NAME",
                self.friendly_name.clone().unwrap_or_default(),
            ),
            ("DOCKER IMAGE", self.name.clone().unwrap_or_default()),
            ("DESCRIPTION", self.description.clone().unwrap_or_default()),
            (
                "ENABLED",
                self.enabled.map(|v| v.to_string()).unwrap_or_default(),
            ),
            (
                "CORES",
                self.cores.map(|v| v.to_string()).unwrap_or_default(),
            ),
            ("MEMORY", self.memory.map(format_bytes).unwrap_or_default()),
            ("IMAGE SRC", self.image_src.clone().unwrap_or_default()),
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
