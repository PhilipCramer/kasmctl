use kasmctl::api::KasmClient;

use super::helpers::{discover_image_id, require_integration_env};

#[test]
fn get_images_returns_list() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let images = client.get_images().expect("get_images should succeed");

    assert!(!images.is_empty(), "server should have at least one image");
}

#[test]
fn get_images_have_required_fields() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let images = client.get_images().expect("get_images should succeed");

    assert!(
        !images.is_empty(),
        "need at least one image for field checks"
    );

    let first = &images[0];
    assert!(
        !first.image_id.is_empty(),
        "first image should have a non-empty image_id"
    );

    let has_friendly_name = images.iter().any(|img| img.friendly_name.is_some());
    assert!(
        has_friendly_name,
        "at least one image should have a friendly_name"
    );
}

#[test]
fn get_images_contains_enabled_image() {
    let ctx = require_integration_env!();
    let client = KasmClient::new(&ctx).unwrap();

    let image_id = discover_image_id(&client).expect("should find at least one enabled image");

    assert!(
        !image_id.is_empty(),
        "discovered image_id should not be empty"
    );
}
