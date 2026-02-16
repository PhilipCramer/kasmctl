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

    /// Core POST helper — `path` is appended to `/api/` (e.g. `"public/get_kasms"` or `"stop_kasm"`).
    fn post<Req, Resp>(&self, path: &str, body: &Req) -> Result<Resp>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model::Context;

    fn test_context(server_url: &str) -> Context {
        Context {
            server: server_url.to_string(),
            api_key: "test-key".into(),
            api_secret: "test-secret".into(),
            insecure_skip_tls_verify: false,
            timeout_seconds: None,
        }
    }

    #[derive(Serialize)]
    struct DummyRequest {
        target_user: String,
    }

    #[derive(Deserialize, Debug)]
    struct DummyResponse {
        ok: bool,
    }

    #[test]
    fn post_admin_routes_to_admin_prefix() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/admin/get_users")
            .with_status(200)
            .with_body(r#"{"ok":true}"#)
            .create();

        let ctx = test_context(&server.url());
        let client = KasmClient::new(&ctx).unwrap();
        let resp: DummyResponse = client
            .post(
                "admin/get_users",
                &DummyRequest {
                    target_user: "u1".into(),
                },
            )
            .unwrap();

        assert!(resp.ok);
        mock.assert();
    }

    #[test]
    fn post_admin_injects_auth_credentials() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/admin/get_users")
            .match_body(mockito::Matcher::PartialJsonString(
                r#"{"api_key":"test-key","api_key_secret":"test-secret"}"#.into(),
            ))
            .with_status(200)
            .with_body(r#"{"ok":true}"#)
            .create();

        let ctx = test_context(&server.url());
        let client = KasmClient::new(&ctx).unwrap();
        let _: DummyResponse = client
            .post(
                "admin/get_users",
                &DummyRequest {
                    target_user: "u1".into(),
                },
            )
            .unwrap();

        mock.assert();
    }

    #[test]
    fn post_admin_forwards_request_body() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/admin/get_users")
            .match_body(mockito::Matcher::PartialJsonString(
                r#"{"target_user":"u1"}"#.into(),
            ))
            .with_status(200)
            .with_body(r#"{"ok":true}"#)
            .create();

        let ctx = test_context(&server.url());
        let client = KasmClient::new(&ctx).unwrap();
        let _: DummyResponse = client
            .post(
                "admin/get_users",
                &DummyRequest {
                    target_user: "u1".into(),
                },
            )
            .unwrap();

        mock.assert();
    }

    #[test]
    fn post_admin_error_message_in_response() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/api/admin/get_users")
            .with_status(200)
            .with_body(r#"{"error_message":"permission denied"}"#)
            .create();

        let ctx = test_context(&server.url());
        let client = KasmClient::new(&ctx).unwrap();
        let result: Result<DummyResponse> = client.post(
            "admin/get_users",
            &DummyRequest {
                target_user: "u1".into(),
            },
        );

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("permission denied"), "error was: {err}");

        mock.assert();
    }
}
