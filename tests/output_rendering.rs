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
    fn table_output_contains_all_headers(session in arb_session()) {
        let output = output::render_one(&session, &OutputFormat::Table).unwrap();
        for header in &["KASM ID", "STATUS", "IMAGE", "USER", "CREATED"] {
            prop_assert!(output.contains(header), "missing header: {}", header);
        }
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
