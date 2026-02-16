use kasmctl::api::KasmClient;
use kasmctl::config::model::Context;

use super::helpers::{NONEXISTENT_UUID, require_integration_env};

/// Requests with invalid API credentials should fail with an error.
#[test]
fn invalid_credentials_returns_error() {
    let ctx = require_integration_env!();

    // Use the real server URL but with bogus credentials.
    let bad_ctx = Context {
        server: ctx.server,
        api_key: "bogus-api-key".to_string(),
        api_secret: "bogus-api-secret".to_string(),
        insecure_skip_tls_verify: ctx.insecure_skip_tls_verify,
        timeout_seconds: None,
    };
    let client = KasmClient::new(&bad_ctx).unwrap();

    let result = client.get_kasms();

    assert!(
        result.is_err(),
        "get_kasms with invalid credentials should return an error"
    );
}

/// Requests to a nonexistent server should fail with a connection error.
#[test]
fn invalid_server_url_returns_connection_error() {
    let bad_ctx = Context {
        server: "https://nonexistent.invalid".to_string(),
        api_key: "irrelevant".to_string(),
        api_secret: "irrelevant".to_string(),
        insecure_skip_tls_verify: false,
        timeout_seconds: None,
    };
    let client = KasmClient::new(&bad_ctx).unwrap();

    let result = client.get_kasms();

    assert!(
        result.is_err(),
        "get_kasms against a nonexistent server should return an error"
    );
}

/// Stopping a nonexistent session should return an error.
#[test]
fn stop_nonexistent_session_returns_error() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let result = client.stop_kasm(NONEXISTENT_UUID);

    assert!(
        result.is_err(),
        "stop_kasm with a nonexistent ID should return an error"
    );
}

/// Pausing a nonexistent session should return an error.
#[test]
fn pause_nonexistent_session_returns_error() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let result = client.pause_kasm(NONEXISTENT_UUID);

    assert!(
        result.is_err(),
        "pause_kasm with a nonexistent ID should return an error"
    );
}

/// Resuming a nonexistent session should return an error.
#[test]
fn resume_nonexistent_session_returns_error() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let result = client.resume_kasm(NONEXISTENT_UUID);

    assert!(
        result.is_err(),
        "resume_kasm with a nonexistent ID should return an error"
    );
}
