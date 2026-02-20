use kasmctl::models::image::Image;
use kasmctl::models::session::{CreateSessionResponse, Session};
use kasmctl::resource::Resource;
use proptest::prelude::*;

fn arb_option_string() -> impl Strategy<Value = Option<String>> {
    prop_oneof![Just(None), "[a-zA-Z0-9 _-]{0,50}".prop_map(Some),]
}

fn arb_option_bool() -> impl Strategy<Value = Option<bool>> {
    prop_oneof![Just(None), any::<bool>().prop_map(Some),]
}

fn arb_option_f64() -> impl Strategy<Value = Option<f64>> {
    prop_oneof![Just(None), (10..160u32).prop_map(|v| Some(v as f64 / 10.0)),]
}

fn arb_option_memory() -> impl Strategy<Value = Option<i64>> {
    prop_oneof![
        Just(None),
        prop_oneof![
            Just(1_073_741_824i64), // 1GB
            Just(2_147_483_648i64), // 2GB
            Just(4_294_967_296i64), // 4GB
            Just(524_288_000i64),   // 500MB
            Just(1_048_576i64),     // 1MB
            (1i64..10_000_000_000i64),
        ]
        .prop_map(Some),
    ]
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

fn arb_session() -> impl Strategy<Value = Session> {
    (
        (
            "[a-zA-Z0-9-]{1,36}",
            arb_option_string(),
            arb_option_string(),
            arb_option_string(),
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
        ),
    )
        .prop_map(
            |(
                (
                    kasm_id,
                    user_id,
                    image_id,
                    username,
                    share_id,
                    kasm_url,
                    created_date,
                    expiration_date,
                ),
                (hostname, server_id, keepalive_date, start_date, operational_status, container_id),
            )| {
                Session {
                    kasm_id,
                    user_id,
                    image_id,
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
        // session_token is intentionally not serialized (it is a write-only server field),
        // so it is always absent from outgoing JSON and becomes None after roundtrip.
        let expected = CreateSessionResponse { session_token: None, ..resp };
        prop_assert_eq!(expected, deserialized);
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
        if session.operational_status.is_none() {
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
        if session.operational_status.is_none() {
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
        if session.hostname.is_none() {
            let val = lookup("HOSTNAME");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.server_id.is_none() {
            let val = lookup("SERVER ID");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.keepalive_date.is_none() {
            let val = lookup("KEEPALIVE");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.start_date.is_none() {
            let val = lookup("STARTED");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.operational_status.is_none() {
            let val = lookup("STATUS");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if session.container_id.is_none() {
            let val = lookup("CONTAINER ID");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
    }

    // --- Image ---

    #[test]
    fn image_serde_roundtrip(image in arb_image()) {
        let json = serde_json::to_string(&image).unwrap();
        let deserialized: Image = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(image, deserialized);
    }

    #[test]
    fn image_table_row_length_matches_headers(image in arb_image()) {
        let headers = Image::table_headers();
        let row = image.table_row();
        prop_assert_eq!(row.len(), headers.len());
    }

    #[test]
    fn image_table_row_first_column_is_image_id(image in arb_image()) {
        let row = image.table_row();
        prop_assert_eq!(&row[0], &image.image_id);
    }

    #[test]
    fn image_table_row_none_fields_become_empty_string(image in arb_image()) {
        let row = image.table_row();
        if image.friendly_name.is_none() {
            prop_assert_eq!(&row[1], "");
        }
        if image.enabled.is_none() {
            prop_assert_eq!(&row[2], "");
        }
        if image.cores.is_none() {
            prop_assert_eq!(&row[3], "");
        }
        if image.memory.is_none() {
            prop_assert_eq!(&row[4], "");
        }
    }

    #[test]
    fn image_table_detail_contains_image_id(image in arb_image()) {
        let detail = image.table_detail();
        let entry = detail.iter().find(|(k, _)| *k == "IMAGE ID");
        prop_assert!(entry.is_some());
        prop_assert_eq!(&entry.unwrap().1, &image.image_id);
    }

    #[test]
    fn image_table_detail_none_fields_become_empty_string(image in arb_image()) {
        let detail = image.table_detail();
        let lookup = |label: &str| detail.iter().find(|(k, _)| *k == label).map(|(_, v)| v.clone());
        if image.friendly_name.is_none() {
            let val = lookup("FRIENDLY NAME");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if image.name.is_none() {
            let val = lookup("DOCKER IMAGE");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if image.description.is_none() {
            let val = lookup("DESCRIPTION");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if image.enabled.is_none() {
            let val = lookup("ENABLED");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if image.cores.is_none() {
            let val = lookup("CORES");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if image.memory.is_none() {
            let val = lookup("MEMORY");
            prop_assert_eq!(val.as_deref(), Some(""));
        }
        if image.image_src.is_none() {
            let val = lookup("IMAGE SRC");
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
        "KASM ID",
        "STATUS",
        "IMAGE ID",
        "USERNAME",
        "USER ID",
        "HOSTNAME",
        "SERVER ID",
        "CONTAINER ID",
        "SHARE ID",
        "KASM URL",
        "STARTED",
        "KEEPALIVE",
        "CREATED",
        "EXPIRES",
    ];
    let session = Session {
        kasm_id: "test-id".into(),
        user_id: None,
        image_id: None,
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
    let detail = session.table_detail();
    let labels: Vec<&str> = detail.iter().map(|(k, _)| *k).collect();
    assert_eq!(labels, expected_labels);
}

#[test]
fn table_detail_contains_field_values() {
    let session = Session {
        kasm_id: "abc-123".into(),
        user_id: Some("user-456".into()),
        image_id: Some("img-789".into()),
        username: Some("alice".into()),
        share_id: Some("share-001".into()),
        kasm_url: Some("https://kasm.example.com/session".into()),
        created_date: Some("2026-01-01T00:00:00Z".into()),
        expiration_date: Some("2026-01-02T00:00:00Z".into()),
        hostname: Some("kasm-host-01".into()),
        server_id: Some("server-abc".into()),
        keepalive_date: Some("2026-01-01T12:00:00Z".into()),
        start_date: Some("2026-01-01T00:05:00Z".into()),
        operational_status: Some("running".into()),
        container_id: Some("container-xyz".into()),
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
    assert_eq!(lookup("HOSTNAME"), "kasm-host-01");
    assert_eq!(lookup("SERVER ID"), "server-abc");
    assert_eq!(lookup("CONTAINER ID"), "container-xyz");
    assert_eq!(lookup("SHARE ID"), "share-001");
    assert_eq!(lookup("KASM URL"), "https://kasm.example.com/session");
    assert_eq!(lookup("STARTED"), "2026-01-01T00:05:00Z");
    assert_eq!(lookup("KEEPALIVE"), "2026-01-01T12:00:00Z");
    assert_eq!(lookup("CREATED"), "2026-01-01T00:00:00Z");
    assert_eq!(lookup("EXPIRES"), "2026-01-02T00:00:00Z");
}

#[test]
fn deserialize_missing_required_kasm_id_fails() {
    let json = r#"{"operational_status": "running"}"#;
    let result = serde_json::from_str::<Session>(json);
    assert!(result.is_err());
}

// ===================== Image =====================

#[test]
fn image_resource_name_is_image() {
    assert_eq!(Image::resource_name(), "Image");
}

#[test]
fn image_table_headers_are_correct() {
    assert_eq!(
        Image::table_headers(),
        vec!["IMAGE ID", "NAME", "ENABLED", "CORES", "MEMORY"]
    );
}

// --- Image table_detail ---

#[test]
fn image_table_detail_has_all_labels() {
    let expected_labels = vec![
        "IMAGE ID",
        "FRIENDLY NAME",
        "DOCKER IMAGE",
        "DESCRIPTION",
        "ENABLED",
        "CORES",
        "MEMORY",
        "IMAGE SRC",
    ];
    let image = Image {
        image_id: "test-id".into(),
        friendly_name: None,
        name: None,
        description: None,
        enabled: None,
        cores: None,
        memory: None,
        image_src: None,
    };
    let detail = image.table_detail();
    let labels: Vec<&str> = detail.iter().map(|(k, _)| *k).collect();
    assert_eq!(labels, expected_labels);
}

#[test]
fn image_table_detail_contains_field_values() {
    let image = Image {
        image_id: "img-abc".into(),
        friendly_name: Some("Ubuntu Desktop".into()),
        name: Some("kasmweb/ubuntu-focal:1.14.0".into()),
        description: Some("Ubuntu 20.04 LTS desktop".into()),
        enabled: Some(true),
        cores: Some(2.0),
        memory: Some(2_147_483_648),
        image_src: Some("https://kasm.example.com/img.png".into()),
    };
    let detail = image.table_detail();
    let lookup = |label: &str| {
        detail
            .iter()
            .find(|(k, _)| *k == label)
            .map(|(_, v)| v.clone())
            .unwrap()
    };
    assert_eq!(lookup("IMAGE ID"), "img-abc");
    assert_eq!(lookup("FRIENDLY NAME"), "Ubuntu Desktop");
    assert_eq!(lookup("DOCKER IMAGE"), "kasmweb/ubuntu-focal:1.14.0");
    assert_eq!(lookup("DESCRIPTION"), "Ubuntu 20.04 LTS desktop");
    assert_eq!(lookup("ENABLED"), "true");
    assert_eq!(lookup("CORES"), "2");
    assert_eq!(lookup("MEMORY"), "2GB");
    assert_eq!(lookup("IMAGE SRC"), "https://kasm.example.com/img.png");
}

#[test]
fn deserialize_missing_required_image_id_fails() {
    let json = r#"{"friendly_name": "Ubuntu"}"#;
    let result = serde_json::from_str::<Image>(json);
    assert!(result.is_err());
}

#[test]
fn image_memory_format_bytes_gb() {
    let image = Image {
        image_id: "test".into(),
        friendly_name: None,
        name: None,
        description: None,
        enabled: None,
        cores: None,
        memory: Some(4_294_967_296), // 4GB
        image_src: None,
    };
    let row = image.table_row();
    assert_eq!(row[4], "4GB");
}

#[test]
fn image_memory_format_bytes_mb() {
    let image = Image {
        image_id: "test".into(),
        friendly_name: None,
        name: None,
        description: None,
        enabled: None,
        cores: None,
        memory: Some(524_288_000), // 500MB
        image_src: None,
    };
    let row = image.table_row();
    assert_eq!(row[4], "500MB");
}

#[test]
fn image_memory_format_bytes_raw() {
    let image = Image {
        image_id: "test".into(),
        friendly_name: None,
        name: None,
        description: None,
        enabled: None,
        cores: None,
        memory: Some(12345),
        image_src: None,
    };
    let row = image.table_row();
    assert_eq!(row[4], "12345");
}
