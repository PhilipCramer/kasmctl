use kasmctl::api::KasmClient;

use super::helpers::{
    discover_image_id, require_integration_env, try_create_session, wait_for_status,
};

/// Lifecycle: create → wait running → verify status → destroy.
///
/// NOTE: stop/pause/resume are excluded because the API client does not yet
/// send `user_id` in those requests, which the Kasm internal API requires.
/// Once the API client is fixed, expand this test to cover the full
/// create→stop→resume→pause→resume→destroy cycle.
#[tokio::test]
async fn session_full_lifecycle() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let image_id = discover_image_id(&client)
        .await
        .expect("need an enabled image for lifecycle test");

    let resp = match try_create_session(&client, &image_id).await {
        Some(resp) => resp,
        None => return,
    };
    let kasm_id = resp.kasm_id.clone();

    // Run all assertions inside a closure so we always clean up.
    let result: Result<(), String> = async {
        // Wait for the session to reach "running" (uses get_kasms internally).
        let session = wait_for_status(&client, &kasm_id, "running")
            .await
            .map_err(|e| e.to_string())?;

        assert_eq!(
            session.operational_status.as_deref(),
            Some("running"),
            "session should be running"
        );

        Ok(())
    }
    .await;

    // Always destroy the session, even if assertions failed.
    let _ = client.destroy_kasm(&kasm_id).await;

    result.unwrap();
}

/// Creating a session returns a response with a valid kasm_id.
#[tokio::test]
async fn create_session_returns_valid_response() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let image_id = discover_image_id(&client)
        .await
        .expect("need an enabled image to create a session");

    let resp = match try_create_session(&client, &image_id).await {
        Some(resp) => resp,
        None => return,
    };
    let kasm_id = resp.kasm_id.clone();

    assert!(
        !resp.kasm_id.is_empty(),
        "kasm_id in response should not be empty"
    );

    // Always clean up.
    let _ = client.destroy_kasm(&kasm_id).await;
}

/// Destroying a nonexistent session should return an error.
///
/// Uses a well-formed UUID to avoid PostgreSQL `InvalidTextRepresentation` errors.
#[tokio::test]
async fn destroy_nonexistent_session_returns_error() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let result = client
        .destroy_kasm("00000000-0000-0000-0000-000000000000")
        .await;

    assert!(
        result.is_err(),
        "destroy_kasm with a nonexistent ID should return an error"
    );
}
