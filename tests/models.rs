use kasmctl::models::session::{CreateSessionResponse, Session};
use kasmctl::resource::Resource;
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

fn arb_create_session_response() -> impl Strategy<Value = CreateSessionResponse> {
    (
        "[a-zA-Z0-9-]{1,36}",
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
        arb_option_string(),
    )
        .prop_map(
            |(kasm_id, status, kasm_url, session_token, share_id, user_id, username)| {
                CreateSessionResponse {
                    kasm_id,
                    status,
                    kasm_url,
                    session_token,
                    share_id,
                    user_id,
                    username,
                }
            },
        )
}

proptest! {
    #[test]
    fn session_serde_roundtrip(session in arb_session()) {
        let json = serde_json::to_string(&session).unwrap();
        let deserialized: Session = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(session, deserialized);
    }

    #[test]
    fn create_session_response_serde_roundtrip(resp in arb_create_session_response()) {
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: CreateSessionResponse = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(resp, deserialized);
    }

    #[test]
    fn table_row_length_matches_headers(session in arb_session()) {
        let headers = Session::table_headers();
        let row = session.table_row();
        prop_assert_eq!(row.len(), headers.len());
    }

    #[test]
    fn table_row_first_column_is_kasm_id(session in arb_session()) {
        let row = session.table_row();
        prop_assert_eq!(&row[0], &session.kasm_id);
    }

    #[test]
    fn table_row_none_fields_become_empty_string(session in arb_session()) {
        let row = session.table_row();
        if session.status.is_none() {
            prop_assert_eq!(&row[1], "");
        }
        if session.image_id.is_none() {
            prop_assert_eq!(&row[2], "");
        }
        if session.username.is_none() {
            prop_assert_eq!(&row[3], "");
        }
        if session.created_date.is_none() {
            prop_assert_eq!(&row[4], "");
        }
    }

    // --- table_detail ---

    #[test]
    fn table_detail_contains_kasm_id(session in arb_session()) {
        let detail = session.table_detail();
        let kasm_entry = detail.iter().find(|(k, _)| *k == "KASM ID");
        prop_assert!(kasm_entry.is_some());
        prop_assert_eq!(&kasm_entry.unwrap().1, &session.kasm_id);
    }

    #[test]
    fn table_detail_none_fields_become_empty_string(session in arb_session()) {
        let detail = session.table_detail();
        let lookup = |label: &str| detail.iter().find(|(k, _)| *k == label).map(|(_, v)| v.clone());
        if session.status.is_none() {
            let val = lookup("STATUS");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.user_id.is_none() {
            let val = lookup("USER ID");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.share_id.is_none() {
            let val = lookup("SHARE ID");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.kasm_url.is_none() {
            let val = lookup("KASM URL");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.expiration_date.is_none() {
            let val = lookup("EXPIRES");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
    }
}

#[test]
fn resource_name_is_session() {
    assert_eq!(Session::resource_name(), "Session");
}

#[test]
fn table_headers_are_correct() {
    assert_eq!(
        Session::table_headers(),
        vec!["KASM ID", "STATUS", "IMAGE", "USER", "CREATED"]
    );
}

// --- table_detail ---

#[test]
fn table_detail_has_all_labels() {
    let expected_labels = vec![
        "KASM ID", "STATUS", "IMAGE ID", "USERNAME", "USER ID", "SHARE ID", "KASM URL", "CREATED",
        "EXPIRES",
    ];
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
    let detail = session.table_detail();
    let labels: Vec<&str> = detail.iter().map(|(k, _)| *k).collect();
    assert_eq!(labels, expected_labels);
}

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
    let detail = session.table_detail();
    let lookup = |label: &str| {
        detail
            .iter()
            .find(|(k, _)| *k == label)
            .map(|(_, v)| v.clone())
            .unwrap()
    };
    assert_eq!(lookup("KASM ID"), "abc-123");
    assert_eq!(lookup("STATUS"), "running");
    assert_eq!(lookup("IMAGE ID"), "img-789");
    assert_eq!(lookup("USERNAME"), "alice");
    assert_eq!(lookup("USER ID"), "user-456");
    assert_eq!(lookup("SHARE ID"), "share-001");
    assert_eq!(lookup("KASM URL"), "https://kasm.example.com/session");
    assert_eq!(lookup("CREATED"), "2026-01-01T00:00:00Z");
    assert_eq!(lookup("EXPIRES"), "2026-01-02T00:00:00Z");
}

#[test]
fn deserialize_missing_required_kasm_id_fails() {
    let json = r#"{"status": "running"}"#;
    let result = serde_json::from_str::<Session>(json);
    assert!(result.is_err());
}
