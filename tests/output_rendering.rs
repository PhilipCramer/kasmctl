use kasmctl::models::session::Session;
use kasmctl::output::{self, OutputFormat};
use proptest::prelude::*;

fn arb_option_string() -> impl Strategy<Value = Option<String>> {
    prop_oneof![Just(None), "[a-zA-Z0-9 _-]{0,50}".prop_map(Some),]
}

fn arb_session() -> impl Strategy<Value = Session> {
    (
        "[a-zA-Z0-9-]{1,36}",
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
    )
        .prop_map(
            |(
                kasm_id,
                user_id,
                status,
                image_id,
                username,
                share_id,
                kasm_url,
                created_date,
                expiration_date,
            )| {
                Session {
                    kasm_id,
                    user_id,
                    status,
                    image_id,
                    username,
                    share_id,
                    kasm_url,
                    created_date,
                    expiration_date,
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
            "KASM ID", "STATUS", "IMAGE ID", "USERNAME", "USER ID",
            "SHARE ID", "KASM URL", "CREATED", "EXPIRES",
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
                output.contains(&session.kasm_id),
                "missing kasm_id: {}",
                session.kasm_id
            );
        }
    }
}

#[test]
fn table_empty_list_has_headers_only() {
    let output = output::render_list::<Session>(&[], &OutputFormat::Table).unwrap();
    assert!(output.contains("KASM ID"));
    // No data rows means no UUIDs — just headers and borders
    assert!(!output.contains("running"));
}

// --- Detail view rendering ---

#[test]
fn table_detail_contains_field_values() {
    let session = Session {
        kasm_id: "abc-123".into(),
        user_id: Some("user-456".into()),
        status: Some("running".into()),
        image_id: Some("img-789".into()),
        username: Some("alice".into()),
        share_id: Some("share-001".into()),
        kasm_url: Some("https://kasm.example.com/session".into()),
        created_date: Some("2026-01-01T00:00:00Z".into()),
        expiration_date: Some("2026-01-02T00:00:00Z".into()),
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
        status: None,
        image_id: None,
        username: None,
        share_id: None,
        kasm_url: None,
        created_date: None,
        expiration_date: None,
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
        status: Some("running".into()),
        image_id: Some("img-789".into()),
        username: Some("alice".into()),
        share_id: Some("share-001".into()),
        kasm_url: Some("https://kasm.example.com/session".into()),
        created_date: Some("2026-01-01T00:00:00Z".into()),
        expiration_date: Some("2026-01-02T00:00:00Z".into()),
    };
    let output = output::render_list(&[session], &OutputFormat::Table).unwrap();
    // List view uses compact 5-column headers
    assert!(output.contains("KASM ID"));
    assert!(output.contains("STATUS"));
    assert!(output.contains("IMAGE"));
    assert!(output.contains("USER"));
    assert!(output.contains("CREATED"));
    // List view should NOT contain detail-only labels
    assert!(!output.contains("SHARE ID"));
    assert!(!output.contains("EXPIRES"));
}

#[test]
fn json_render_one_is_pretty_printed() {
    let session = Session {
        kasm_id: "test-id".into(),
        user_id: None,
        status: Some("running".into()),
        image_id: None,
        username: None,
        share_id: None,
        kasm_url: None,
        created_date: None,
        expiration_date: None,
    };
    let output = output::render_one(&session, &OutputFormat::Json).unwrap();
    assert!(output.contains('\n'), "expected pretty-printed JSON");
}
