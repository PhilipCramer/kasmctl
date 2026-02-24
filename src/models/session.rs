use serde::{Deserialize, Serialize};

use crate::output::display::{relative_age, short_id};
use crate::resource::Resource;

/// Nested image metadata returned alongside a session.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionImage {
    #[serde(default)]
    pub friendly_name: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub kasm_id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub image: Option<SessionImage>,
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
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub server_id: Option<String>,
    #[serde(default)]
    pub keepalive_date: Option<String>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub operational_status: Option<String>,
    #[serde(default)]
    pub container_id: Option<String>,
}

impl Resource for Session {
    fn resource_name() -> &'static str {
        "Session"
    }

    fn table_headers() -> Vec<&'static str> {
        vec!["KASM ID", "STATUS", "IMAGE", "USER", "AGE"]
    }

    fn table_row(&self) -> Vec<String> {
        let image_display = self
            .image
            .as_ref()
            .and_then(|i| i.friendly_name.as_deref().or(i.name.as_deref()))
            .or(self.image_id.as_deref())
            .unwrap_or_default()
            .to_string();

        vec![
            short_id(&self.kasm_id).to_string(),
            self.operational_status.clone().unwrap_or_default(),
            image_display,
            self.username.clone().unwrap_or_default(),
            self.created_date
                .as_deref()
                .map(relative_age)
                .unwrap_or_default(),
        ]
    }

    fn table_detail(&self) -> Vec<(&'static str, String)> {
        let image_friendly = self
            .image
            .as_ref()
            .and_then(|i| i.friendly_name.as_deref().or(i.name.as_deref()))
            .unwrap_or_default()
            .to_string();

        vec![
            ("KASM ID", self.kasm_id.clone()),
            (
                "STATUS",
                self.operational_status.clone().unwrap_or_default(),
            ),
            ("IMAGE", image_friendly),
            ("IMAGE ID", self.image_id.clone().unwrap_or_default()),
            ("USERNAME", self.username.clone().unwrap_or_default()),
            ("USER ID", self.user_id.clone().unwrap_or_default()),
            ("HOSTNAME", self.hostname.clone().unwrap_or_default()),
            ("SERVER ID", self.server_id.clone().unwrap_or_default()),
            (
                "CONTAINER ID",
                self.container_id.clone().unwrap_or_default(),
            ),
            ("SHARE ID", self.share_id.clone().unwrap_or_default()),
            ("KASM URL", self.kasm_url.clone().unwrap_or_default()),
            ("STARTED", self.start_date.clone().unwrap_or_default()),
            ("KEEPALIVE", self.keepalive_date.clone().unwrap_or_default()),
            ("CREATED", self.created_date.clone().unwrap_or_default()),
            ("EXPIRES", self.expiration_date.clone().unwrap_or_default()),
        ]
    }
}

/// Response from the `request_kasm` endpoint.
#[derive(PartialEq, Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub kasm_id: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub kasm_url: Option<String>,
    #[serde(default, skip_serializing)]
    pub session_token: Option<String>,
    #[serde(default)]
    pub share_id: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}

impl std::fmt::Debug for CreateSessionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateSessionResponse")
            .field("kasm_id", &self.kasm_id)
            .field("status", &self.status)
            .field("kasm_url", &self.kasm_url)
            .field(
                "session_token",
                &self.session_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("share_id", &self.share_id)
            .field("user_id", &self.user_id)
            .field("username", &self.username)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_session_response_does_not_serialize_session_token() {
        let resp = CreateSessionResponse {
            kasm_id: "abc123".into(),
            session_token: Some("super-secret-token".into()),
            status: None,
            kasm_url: None,
            share_id: None,
            user_id: None,
            username: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(
            !json.contains("session_token"),
            "session_token field must not appear in JSON output"
        );
        assert!(
            !json.contains("super-secret-token"),
            "token value must not appear in JSON output"
        );
    }
}
