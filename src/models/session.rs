use serde::{Deserialize, Serialize};

use crate::resource::Resource;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub kasm_id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub share_id: Option<String>,
    #[serde(default)]
    pub kasm_url: Option<String>,
    #[serde(default)]
    pub created_date: Option<String>,
    #[serde(default)]
    pub expiration_date: Option<String>,
}

impl Resource for Session {
    fn resource_name() -> &'static str {
        "Session"
    }

    fn table_headers() -> Vec<&'static str> {
        vec!["KASM ID", "STATUS", "IMAGE", "USER", "CREATED"]
    }

    fn table_row(&self) -> Vec<String> {
        vec![
            self.kasm_id.clone(),
            self.status.clone().unwrap_or_default(),
            self.image_id.clone().unwrap_or_default(),
            self.username.clone().unwrap_or_default(),
            self.created_date.clone().unwrap_or_default(),
        ]
    }

    fn table_detail(&self) -> Vec<(&'static str, String)> {
        vec![
            ("KASM ID", self.kasm_id.clone()),
            ("STATUS", self.status.clone().unwrap_or_default()),
            ("IMAGE ID", self.image_id.clone().unwrap_or_default()),
            ("USERNAME", self.username.clone().unwrap_or_default()),
            ("USER ID", self.user_id.clone().unwrap_or_default()),
            ("SHARE ID", self.share_id.clone().unwrap_or_default()),
            ("KASM URL", self.kasm_url.clone().unwrap_or_default()),
            ("CREATED", self.created_date.clone().unwrap_or_default()),
            ("EXPIRES", self.expiration_date.clone().unwrap_or_default()),
        ]
    }
}

/// Response from the `request_kasm` endpoint.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub kasm_id: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub kasm_url: Option<String>,
    #[serde(default)]
    pub session_token: Option<String>,
    #[serde(default)]
    pub share_id: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}
