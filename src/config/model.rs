use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KasmConfig {
    #[serde(rename = "current-context", skip_serializing_if = "Option::is_none")]
    pub current_context: Option<String>,
    #[serde(default)]
    pub contexts: Vec<NamedContext>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NamedContext {
    pub name: String,
    #[serde(flatten)]
    pub context: Context,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Context {
    pub server: String,
    #[serde(rename = "api-key")]
    pub api_key: String,
    #[serde(rename = "api-secret")]
    pub api_secret: String,
    #[serde(
        default,
        rename = "insecure-skip-tls-verify",
        skip_serializing_if = "std::ops::Not::not"
    )]
    pub insecure_skip_tls_verify: bool,
    #[serde(
        default,
        rename = "timeout-seconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_seconds: Option<u64>,
}
