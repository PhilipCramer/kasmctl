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

#[test]
fn deserialize_missing_required_kasm_id_fails() {
    let json = r#"{"status": "running"}"#;
    let result = serde_json::from_str::<Session>(json);
    assert!(result.is_err());
}
