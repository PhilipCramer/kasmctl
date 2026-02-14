pub mod error;
pub mod sessions;

use anyhow::{Context as _, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::model::Context;

use self::error::ApiError;

pub struct KasmClient {
    http: Client,
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
        let mut builder = Client::builder();
        if context.insecure_skip_tls_verify {
            builder = builder.danger_accept_invalid_certs(true);
        }
        let http = builder.build().context("failed to create HTTP client")?;
        Ok(Self {
            http,
            base_url: context.server.trim_end_matches('/').to_string(),
            api_key: context.api_key.clone(),
            api_secret: context.api_secret.clone(),
        })
    }

    /// POST to a Kasm API endpoint with auth credentials injected into the body.
    pub(crate) async fn post<Req, Resp>(&self, endpoint: &str, body: &Req) -> Result<Resp>
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

        let url = format!("{}/api/public/{}", self.base_url, endpoint);
        let response = self
            .http
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(ApiError::Connection)?;

        let status = response.status();
        let text = response.text().await.map_err(ApiError::Connection)?;

        // Check for API-level error in response body
        if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(&text)
            && let Some(msg) = err_resp.error_message
        {
            return Err(ApiError::Server {
                status,
                message: msg,
            }
            .into());
        }

        // Check HTTP status
        if !status.is_success() {
            return Err(ApiError::Server {
                status,
                message: format!("HTTP {status}"),
            }
            .into());
        }

        serde_json::from_str(&text).map_err(|e| ApiError::Deserialization(format!("{e}")).into())
    }
}
