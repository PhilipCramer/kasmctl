use std::time::Duration;

use anyhow::{Result, bail};
use tokio::time::{Instant, sleep};

use kasmctl::api::KasmClient;
use kasmctl::config::model::Context;
use kasmctl::config::{load_config, resolve_from_config};
use kasmctl::models::session::Session;

/// Attempt to load the "test" context from the kasmctl config.
///
/// Returns `None` if there is no config file or no context named "test",
/// allowing tests to skip gracefully in environments without a live server.
pub fn integration_context() -> Option<Context> {
    let config = load_config().ok()?;
    resolve_from_config(&config, Some("test")).ok()
}

/// Skip the current test if no "test" context is configured.
///
/// Usage:
/// ```ignore
/// let ctx = require_integration_env!();
/// ```
macro_rules! require_integration_env {
    () => {
        match $crate::helpers::integration_context() {
            Some(ctx) => ctx,
            None => {
                println!("SKIPPED: no 'test' context configured");
                return;
            }
        }
    };
}
pub(crate) use require_integration_env;

/// User ID for the dedicated test user (user@kasm.local).
///
/// Sessions must be created with a user_id; without one, the Kasm API treats
/// them as anonymous and subsequent operations (destroy, stop, etc.) fail.
pub const TEST_USER_ID: &str = "b3aacd1f-bc7b-4912-a6f9-95e891d7a345";

/// Well-formed UUID that does not correspond to any real resource.
///
/// The Kasm database uses UUID columns, so passing non-UUID strings (e.g.
/// `"nonexistent-id"`) causes server-side PostgreSQL `InvalidTextRepresentation`
/// errors. Always use this constant for "nonexistent resource" tests.
pub const NONEXISTENT_UUID: &str = "00000000-0000-0000-0000-000000000000";

/// Transient server errors that should cause a test to skip rather than fail.
const SKIP_PATTERNS: &[&str] = &[
    "No resources are available",
    "license limit exceeded",
    "Unable to create session",
];

/// Try to create a session attributed to [`TEST_USER_ID`], returning `None`
/// (and printing SKIPPED) if the server cannot create a session right now
/// (capacity, licensing, etc.).
pub async fn try_create_session(
    client: &KasmClient,
    image_id: &str,
) -> Option<kasmctl::models::session::CreateSessionResponse> {
    match client.request_kasm(image_id, Some(TEST_USER_ID)).await {
        Ok(resp) => Some(resp),
        Err(e) => {
            let msg = e.to_string();
            if SKIP_PATTERNS.iter().any(|p| msg.contains(p)) {
                println!("SKIPPED: server cannot create session: {msg}");
                None
            } else {
                panic!("request_kasm failed unexpectedly: {e}");
            }
        }
    }
}

/// Find the first enabled image available on the server.
pub async fn discover_image_id(client: &KasmClient) -> Result<String> {
    let images = client.get_images().await?;
    images
        .into_iter()
        .find(|img| img.enabled == Some(true))
        .map(|img| img.image_id)
        .ok_or_else(|| anyhow::anyhow!("no enabled image found on the server"))
}

/// Poll `get_kasm_status` until `operational_status` matches `target`.
///
/// Checks every 3 seconds with a timeout of 120 seconds.
pub async fn wait_for_status(client: &KasmClient, kasm_id: &str, target: &str) -> Result<Session> {
    let start = Instant::now();
    let timeout = Duration::from_secs(120);
    let interval = Duration::from_secs(3);

    loop {
        let session = client.get_kasm_status(kasm_id, TEST_USER_ID).await?;
        if session.operational_status.as_deref() == Some(target) {
            return Ok(session);
        }
        if start.elapsed() >= timeout {
            bail!(
                "timed out after {}s waiting for kasm {kasm_id} to reach status \"{target}\" \
                 (last status: {:?})",
                timeout.as_secs(),
                session.operational_status,
            );
        }
        sleep(interval).await;
    }
}
