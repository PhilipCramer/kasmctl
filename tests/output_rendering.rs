use kasmctl::models::image::Image;
use kasmctl::models::session::{Session, SessionImage};
use kasmctl::models::zone::Zone;
use kasmctl::output::display::short_id;
use kasmctl::output::{self, OutputFormat};
use proptest::prelude::*;

fn arb_option_string() -> impl Strategy<Value = Option<String>> {
    prop_oneof![Just(None), "[a-zA-Z0-9 _-]{0,50}".prop_map(Some),]
}

fn arb_option_bool() -> impl Strategy<Value = Option<bool>> {
    prop_oneof![Just(None), any::<bool>().prop_map(Some),]
}

fn arb_option_f64() -> impl Strategy<Value = Option<f64>> {
    prop_oneof![Just(None), (1.0..16.0f64).prop_map(Some),]
}

fn arb_option_memory() -> impl Strategy<Value = Option<i64>> {
    prop_oneof![Just(None), (1i64..10_000_000_000i64).prop_map(Some),]
}

fn arb_image() -> impl Strategy<Value = Image> {
    (
        "[a-zA-Z0-9-]{1,36}",
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_bool(),
        arb_option_f64(),
        arb_option_memory(),
        arb_option_string(),
    )
        .prop_map(
            |(image_id, friendly_name, name, description, enabled, cores, memory, image_src)| {
                Image {
                    image_id,
                    friendly_name,
                    name,
                    description,
                    enabled,
                    cores,
                    memory,
                    image_src,
                }
            },
        )
}

fn arb_images() -> impl Strategy<Value = Vec<Image>> {
    prop::collection::vec(arb_image(), 0..10)
}

fn arb_option_session_image() -> impl Strategy<Value = Option<SessionImage>> {
    prop_oneof![
        Just(None),
        (arb_option_string(), arb_option_string()).prop_map(|(friendly_name, name)| Some(
            SessionImage {
                friendly_name,
                name
            }
        ))
    ]
}

fn arb_session() -> impl Strategy<Value = Session> {
    (
        (
            "[a-zA-Z0-9-]{1,36}",
            arb_option_string(),
            arb_option_string(),
            arb_option_session_image(),
            arb_option_string(),
            arb_option_string(),
            arb_option_string(),
            arb_option_string(),
        ),
        (
            arb_option_string(),
            arb_option_string(),
            arb_option_string(),
            arb_option_string(),
            arb_option_string(),
            arb_option_string(),
            arb_option_string(),
        ),
    )
        .prop_map(
            |(
                (kasm_id, user_id, image_id, image, username, share_id, kasm_url, created_date),
                (
                    expiration_date,
                    hostname,
                    server_id,
                    keepalive_date,
                    start_date,
                    operational_status,
                    container_id,
                ),
            )| {
                Session {
                    kasm_id,
                    user_id,
                    image_id,
                    image,
                    username,
                    share_id,
                    kasm_url,
                    created_date,
                    expiration_date,
                    hostname,
                    server_id,
                    keepalive_date,
                    start_date,
                    operational_status,
                    container_id,
                }
            },
        )
}

fn arb_sessions() -> impl Strategy<Value = Vec<Session>> {
    prop::collection::vec(arb_session(), 0..10)
}

proptest! {
    #[test]
    fn json_render_list_produces_valid_json(sessions in arb_sessions()) {
        let output = output::render_list(&sessions, &OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        prop_assert!(parsed.is_array());
    }

    #[test]
    fn json_render_list_length_matches_input(sessions in arb_sessions()) {
        let output = output::render_list(&sessions, &OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        prop_assert_eq!(parsed.as_array().unwrap().len(), sessions.len());
    }

    #[test]
    fn yaml_render_list_produces_valid_yaml(sessions in arb_sessions()) {
        let output = output::render_list(&sessions, &OutputFormat::Yaml).unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(&output).unwrap();
        prop_assert!(parsed.is_sequence());
    }

    #[test]
    fn json_render_one_produces_object(session in arb_session()) {
        let output = output::render_one(&session, &OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        prop_assert!(parsed.is_object());
    }

    #[test]
    fn table_output_contains_kasm_id(session in arb_session()) {
        let output = output::render_one(&session, &OutputFormat::Table).unwrap();
        prop_assert!(output.contains(&session.kasm_id));
    }

    #[test]
    fn table_detail_contains_all_labels(session in arb_session()) {
        let output = output::render_one(&session, &OutputFormat::Table).unwrap();
        for label in &[
            "KASM ID", "STATUS", "IMAGE", "IMAGE ID", "USERNAME", "USER ID",
            "HOSTNAME", "SERVER ID", "CONTAINER ID",
            "SHARE ID", "KASM URL", "STARTED", "KEEPALIVE", "CREATED", "EXPIRES",
        ] {
            prop_assert!(output.contains(label), "missing label: {}", label);
        }
    }

    #[test]
    fn table_detail_contains_kasm_id_value(session in arb_session()) {
        let output = output::render_one(&session, &OutputFormat::Table).unwrap();
        prop_assert!(output.contains(&session.kasm_id));
    }

    #[test]
    fn table_list_contains_all_kasm_ids(sessions in arb_sessions()) {
        let output = output::render_list(&sessions, &OutputFormat::Table).unwrap();
        for session in &sessions {
            prop_assert!(
                output.contains(short_id(&session.kasm_id)),
                "missing short kasm_id: {}",
                short_id(&session.kasm_id)
            );
        }
    }
}

#[test]
fn table_empty_list_has_headers_only() {
    let output = output::render_list::<Session>(&[], &OutputFormat::Table).unwrap();
    assert_eq!(output, "No sessions found.");
}

// --- Detail view rendering ---

#[test]
fn table_detail_contains_field_values() {
    let session = Session {
        kasm_id: "abc-123".into(),
        user_id: Some("user-456".into()),
        image_id: Some("img-789".into()),
        image: None,
        username: Some("alice".into()),
        share_id: Some("share-001".into()),
        kasm_url: Some("https://kasm.example.com/session".into()),
        created_date: Some("2026-01-01T00:00:00Z".into()),
        expiration_date: Some("2026-01-02T00:00:00Z".into()),
        hostname: None,
        server_id: None,
        keepalive_date: None,
        start_date: None,
        operational_status: Some("running".into()),
        container_id: None,
    };
    let output = output::render_one(&session, &OutputFormat::Table).unwrap();
    assert!(output.contains("abc-123"));
    assert!(output.contains("running"));
    assert!(output.contains("img-789"));
    assert!(output.contains("alice"));
    assert!(output.contains("user-456"));
    assert!(output.contains("share-001"));
    assert!(output.contains("https://kasm.example.com/session"));
    assert!(output.contains("2026-01-01T00:00:00Z"));
    assert!(output.contains("2026-01-02T00:00:00Z"));
}

#[test]
fn table_detail_handles_none_fields() {
    let session = Session {
        kasm_id: "test-id".into(),
        user_id: None,
        image_id: None,
        image: None,
        username: None,
        share_id: None,
        kasm_url: None,
        created_date: None,
        expiration_date: None,
        hostname: None,
        server_id: None,
        keepalive_date: None,
        start_date: None,
        operational_status: None,
        container_id: None,
    };
    // Should not panic and should still contain the kasm_id and all labels
    let output = output::render_one(&session, &OutputFormat::Table).unwrap();
    assert!(output.contains("test-id"));
    assert!(output.contains("KASM ID"));
    assert!(output.contains("EXPIRES"));
}

// --- List output unchanged ---

#[test]
fn table_list_still_uses_compact_headers() {
    let session = Session {
        kasm_id: "abc-123".into(),
        user_id: Some("user-456".into()),
        image_id: Some("img-789".into()),
        image: None,
        username: Some("alice".into()),
        share_id: Some("share-001".into()),
        kasm_url: Some("https://kasm.example.com/session".into()),
        created_date: Some("2026-01-01T00:00:00Z".into()),
        expiration_date: Some("2026-01-02T00:00:00Z".into()),
        hostname: None,
        server_id: None,
        keepalive_date: None,
        start_date: None,
        operational_status: Some("running".into()),
        container_id: None,
    };
    let output = output::render_list(&[session], &OutputFormat::Table).unwrap();
    // List view uses compact 5-column headers
    assert!(output.contains("KASM ID"));
    assert!(output.contains("STATUS"));
    assert!(output.contains("IMAGE"));
    assert!(output.contains("USER"));
    assert!(output.contains("AGE"));
    // List view should NOT contain detail-only labels
    assert!(!output.contains("SHARE ID"));
    assert!(!output.contains("EXPIRES"));
}

#[test]
fn json_render_one_is_pretty_printed() {
    let session = Session {
        kasm_id: "test-id".into(),
        user_id: None,
        image_id: None,
        image: None,
        username: None,
        share_id: None,
        kasm_url: None,
        created_date: None,
        expiration_date: None,
        hostname: None,
        server_id: None,
        keepalive_date: None,
        start_date: None,
        operational_status: Some("running".into()),
        container_id: None,
    };
    let output = output::render_one(&session, &OutputFormat::Json).unwrap();
    assert!(output.contains('\n'), "expected pretty-printed JSON");
}

// ===================== Image output rendering =====================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn image_json_render_list_produces_valid_json(images in arb_images()) {
        let output = output::render_list(&images, &OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        prop_assert!(parsed.is_array());
        prop_assert_eq!(parsed.as_array().unwrap().len(), images.len());
    }

    #[test]
    fn image_yaml_render_list_produces_valid_yaml(images in arb_images()) {
        let output = output::render_list(&images, &OutputFormat::Yaml).unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(&output).unwrap();
        prop_assert!(parsed.is_sequence());
    }

    #[test]
    fn image_json_render_one_produces_object(image in arb_image()) {
        let output = output::render_one(&image, &OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        prop_assert!(parsed.is_object());
    }

    #[test]
    fn image_table_list_contains_all_image_ids(images in arb_images()) {
        let output = output::render_list(&images, &OutputFormat::Table).unwrap();
        for image in &images {
            prop_assert!(
                output.contains(short_id(&image.image_id)),
                "missing short image_id: {}",
                short_id(&image.image_id)
            );
        }
    }

    #[test]
    fn image_table_detail_contains_image_id(image in arb_image()) {
        let output = output::render_one(&image, &OutputFormat::Table).unwrap();
        prop_assert!(output.contains(&image.image_id));
    }
}

#[test]
fn image_table_empty_list_has_headers_only() {
    let output = output::render_list::<Image>(&[], &OutputFormat::Table).unwrap();
    assert_eq!(output, "No images found.");
}

// --- Footer count tests ---

#[test]
fn table_list_includes_footer_count_singular() {
    let session = Session {
        kasm_id: "abc-123".into(),
        user_id: None,
        image_id: None,
        image: None,
        username: None,
        share_id: None,
        kasm_url: None,
        created_date: None,
        expiration_date: None,
        hostname: None,
        server_id: None,
        keepalive_date: None,
        start_date: None,
        operational_status: Some("running".into()),
        container_id: None,
    };
    let output = output::render_list(&[session], &OutputFormat::Table).unwrap();
    assert!(output.ends_with("\n1 session"), "output was: {output}");
}

#[test]
fn table_list_includes_footer_count_plural() {
    let make_session = |id: &str| Session {
        kasm_id: id.into(),
        user_id: None,
        image_id: None,
        image: None,
        username: None,
        share_id: None,
        kasm_url: None,
        created_date: None,
        expiration_date: None,
        hostname: None,
        server_id: None,
        keepalive_date: None,
        start_date: None,
        operational_status: None,
        container_id: None,
    };
    let output = output::render_list(
        &[make_session("s1"), make_session("s2")],
        &OutputFormat::Table,
    )
    .unwrap();
    assert!(output.ends_with("\n2 sessions"), "output was: {output}");
}

#[test]
fn image_table_list_includes_footer_count() {
    let image = Image {
        image_id: "img-001".into(),
        friendly_name: Some("Ubuntu Desktop".into()),
        name: None,
        description: None,
        enabled: Some(true),
        cores: None,
        memory: None,
        image_src: None,
    };
    let output = output::render_list(&[image], &OutputFormat::Table).unwrap();
    assert!(output.ends_with("\n1 image"), "output was: {output}");
}

// ===================== Zone output rendering =====================

fn arb_zone() -> impl Strategy<Value = Zone> {
    (
        "[a-zA-Z0-9-]{1,36}",
        prop_oneof![Just(None), "[a-zA-Z0-9 _-]{0,50}".prop_map(Some)],
        prop_oneof![Just(None), "[a-zA-Z0-9 _-]{0,50}".prop_map(Some)],
        prop_oneof![Just(None), "[a-zA-Z0-9 _-]{0,50}".prop_map(Some)],
        prop_oneof![Just(None), "[a-zA-Z0-9 _-]{0,50}".prop_map(Some)],
    )
        .prop_map(
            |(
                zone_id,
                zone_name,
                allow_origin_domain,
                upstream_auth_address,
                load_balancing_strategy,
            )| {
                Zone {
                    zone_id,
                    zone_name,
                    allow_origin_domain,
                    upstream_auth_address,
                    load_balancing_strategy,
                    ..Default::default()
                }
            },
        )
}

fn arb_zones() -> impl Strategy<Value = Vec<Zone>> {
    prop::collection::vec(arb_zone(), 0..10)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn zone_json_render_list_produces_valid_json(zones in arb_zones()) {
        let output = output::render_list(&zones, &OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        prop_assert!(parsed.is_array());
        prop_assert_eq!(parsed.as_array().unwrap().len(), zones.len());
    }

    #[test]
    fn zone_yaml_render_list_produces_valid_yaml(zones in arb_zones()) {
        let output = output::render_list(&zones, &OutputFormat::Yaml).unwrap();
        let parsed: serde_yaml::Value = serde_yaml::from_str(&output).unwrap();
        prop_assert!(parsed.is_sequence());
    }

    #[test]
    fn zone_json_render_one_produces_object(zone in arb_zone()) {
        let output = output::render_one(&zone, &OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        prop_assert!(parsed.is_object());
    }

    #[test]
    fn zone_table_list_contains_all_zone_ids(zones in arb_zones()) {
        let output = output::render_list(&zones, &OutputFormat::Table).unwrap();
        for zone in &zones {
            prop_assert!(
                output.contains(short_id(&zone.zone_id)),
                "missing short zone_id: {}",
                short_id(&zone.zone_id)
            );
        }
    }

    #[test]
    fn zone_table_detail_contains_zone_id(zone in arb_zone()) {
        let output = output::render_one(&zone, &OutputFormat::Table).unwrap();
        prop_assert!(output.contains(&zone.zone_id));
    }
}

#[test]
fn zone_table_empty_list_has_headers_only() {
    let output = output::render_list::<Zone>(&[], &OutputFormat::Table).unwrap();
    assert_eq!(output, "No zones found.");
}

#[test]
fn zone_table_list_includes_footer_count() {
    let zone = Zone {
        zone_id: "zone-001".into(),
        zone_name: Some("us-east".into()),
        ..Default::default()
    };
    let output = output::render_list(&[zone], &OutputFormat::Table).unwrap();
    assert!(output.ends_with("\n1 zone"), "output was: {output}");
}
