pub mod error;
pub mod images;
pub mod sessions;

use std::time::Duration;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use ureq::Agent;
use ureq::tls::TlsConfig;

use crate::config::model::Context;

use self::error::ApiError;

const DEFAULT_TIMEOUT_SECS: u64 = 30;

pub struct KasmClient {
    agent: Agent,
    base_url: String,
    api_key: String,
    api_secret: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error_message: Option<String>,
}

impl KasmClient {
    pub fn new(context: &Context) -> Result<Self> {
        let timeout = Duration::from_secs(context.timeout_seconds.unwrap_or(DEFAULT_TIMEOUT_SECS));

        let mut config_builder = Agent::config_builder()
            .timeout_global(Some(timeout))
            .http_status_as_error(false);

        if context.insecure_skip_tls_verify {
            config_builder =
                config_builder.tls_config(TlsConfig::builder().disable_verification(true).build());
        }

        let agent: Agent = config_builder.build().into();

        Ok(Self {
            agent,
            base_url: context.server.trim_end_matches('/').to_string(),
            api_key: context.api_key.clone(),
            api_secret: context.api_secret.clone(),
        })
    }

    /// POST to a Kasm API endpoint under `/api/public/`.
    pub(crate) fn post<Req, Resp>(&self, endpoint: &str, body: &Req) -> Result<Resp>
    where
        Req: Serialize,
        Resp: for<'de> Deserialize<'de>,
    {
        self.post_raw(&format!("public/{endpoint}"), body)
    }

    /// POST to a Kasm API endpoint under `/api/` (non-public / internal).
    pub(crate) fn post_internal<Req, Resp>(&self, endpoint: &str, body: &Req) -> Result<Resp>
    where
        Req: Serialize,
        Resp: for<'de> Deserialize<'de>,
    {
        self.post_raw(endpoint, body)
    }

    /// Core POST helper — `path` is appended to `/api/` (e.g. `"public/get_kasms"` or `"stop_kasm"`).
    pub(crate) fn post_raw<Req, Resp>(&self, path: &str, body: &Req) -> Result<Resp>
    where
        Req: Serialize,
        Resp: for<'de> Deserialize<'de>,
    {
        let mut payload = serde_json::to_value(body)?;
        let obj = payload
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("request body must be a JSON object"))?;
        obj.insert("api_key".into(), self.api_key.clone().into());
        obj.insert("api_key_secret".into(), self.api_secret.clone().into());

        let url = format!("{}/api/{}", self.base_url, path);
        let response = self
            .agent
            .post(&url)
            .send_json(&payload)
            .map_err(|e| ApiError::Connection(e.to_string()))?;

        let status = response.status().as_u16();
        let body_text = response
            .into_body()
            .read_to_string()
            .map_err(|e| ApiError::Connection(e.to_string()))?;

        // Check for API-level error in response body
        if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(&body_text)
            && let Some(msg) = err_resp.error_message
        {
            return Err(ApiError::Server {
                status,
                message: msg,
            }
            .into());
        }

        // Check HTTP status
        if !(200..300).contains(&status) {
            return Err(ApiError::Server {
                status,
                message: format!("HTTP {status}"),
            }
            .into());
        }

        serde_json::from_str(&body_text)
            .map_err(|e| ApiError::Deserialization(format!("{e}")).into())
    }
}
