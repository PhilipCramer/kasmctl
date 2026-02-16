use kasmctl::api::KasmClient;

use super::helpers::{NONEXISTENT_UUID, TEST_USER_ID, require_integration_env};

#[test]
fn get_kasms_succeeds() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let sessions = client
        .get_kasms()
        .expect("get_kasms should succeed against a live server");

    // The session list may be empty — we only assert the call succeeds.
    let _ = sessions;
}

/// Use a well-formed UUID that doesn't exist to avoid causing PostgreSQL
/// `InvalidTextRepresentation` errors on the server.
#[test]
fn get_kasm_status_nonexistent_id_returns_error() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let result = client.get_kasm_status(NONEXISTENT_UUID, TEST_USER_ID);

    assert!(
        result.is_err(),
        "get_kasm_status with a nonexistent ID should return an error"
    );
}
