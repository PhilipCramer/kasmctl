use kasmctl::api::KasmClient;
use kasmctl::config::model::Context;

fn test_context(server_url: &str) -> Context {
    Context {
        server: server_url.to_string(),
        api_key: "test-key".into(),
        api_secret: "test-secret".into(),
        insecure_skip_tls_verify: false,
    }
}

// --- Auth injection ---

#[tokio::test]
async fn post_injects_api_credentials() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"api_key":"test-key","api_key_secret":"test-secret"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"kasms":[]}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.get_kasms().await.unwrap();

    mock.assert_async().await;
}

// --- get_kasms ---

#[tokio::test]
async fn get_kasms_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(
            r#"{"kasms":[
                {"kasm_id":"abc-123","operational_status":"running"},
                {"kasm_id":"def-456","operational_status":"stopped"}
            ]}"#,
        )
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let sessions = client.get_kasms().await.unwrap();

    assert_eq!(sessions.len(), 2);
    assert_eq!(sessions[0].kasm_id, "abc-123");
    assert_eq!(sessions[1].kasm_id, "def-456");

    mock.assert_async().await;
}

#[tokio::test]
async fn get_kasms_empty_list() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(r#"{"kasms":[]}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let sessions = client.get_kasms().await.unwrap();

    assert!(sessions.is_empty());

    mock.assert_async().await;
}

// --- get_kasm_status ---

#[tokio::test]
async fn get_kasm_status_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_kasm_status")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123","user_id":"user-456"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"kasm":{"kasm_id":"abc-123","operational_status":"running"}}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let session = client.get_kasm_status("abc-123", "user-456").await.unwrap();

    assert_eq!(session.kasm_id, "abc-123");
    assert_eq!(session.operational_status.as_deref(), Some("running"));

    mock.assert_async().await;
}

// --- request_kasm ---

#[tokio::test]
async fn request_kasm_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/request_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"image_id":"img-001"}"#.into(),
        ))
        .with_status(200)
        .with_body(
            r#"{"kasm_id":"new-kasm","status":"starting","kasm_url":"https://kasm.example.com/session"}"#,
        )
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let resp = client.request_kasm("img-001", None).await.unwrap();

    assert_eq!(resp.kasm_id, "new-kasm");
    assert_eq!(resp.status.as_deref(), Some("starting"));
    assert_eq!(
        resp.kasm_url.as_deref(),
        Some("https://kasm.example.com/session")
    );

    mock.assert_async().await;
}

#[tokio::test]
async fn request_kasm_with_user_id() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/request_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"image_id":"img-001","user_id":"user-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"kasm_id":"new-kasm"}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client
        .request_kasm("img-001", Some("user-123"))
        .await
        .unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn request_kasm_without_user_id_omits_field() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/request_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"image_id":"img-001"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"kasm_id":"new-kasm"}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.request_kasm("img-001", None).await.unwrap();

    mock.assert_async().await;
}

// --- destroy_kasm ---

#[tokio::test]
async fn destroy_kasm_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/destroy_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.destroy_kasm("abc-123").await.unwrap();

    mock.assert_async().await;
}

// --- Error handling ---

#[tokio::test]
async fn api_error_message_in_response() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(r#"{"error_message":"unauthorized"}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let result = client.get_kasms().await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("unauthorized"), "error was: {err}");

    mock.assert_async().await;
}

#[tokio::test]
async fn api_http_error_status() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(500)
        .with_body(r#"{"something":"else"}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let result = client.get_kasms().await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("500"), "error was: {err}");

    mock.assert_async().await;
}

#[tokio::test]
async fn api_deserialization_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body("not json at all")
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let result = client.get_kasms().await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("parse") || err.contains("expected"),
        "error was: {err}"
    );

    mock.assert_async().await;
}

// --- URL construction ---

#[tokio::test]
async fn url_construction_with_trailing_slash() {
    let mut server = mockito::Server::new_async().await;
    let url_with_slash = format!("{}/", server.url());

    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(r#"{"kasms":[]}"#)
        .create_async()
        .await;

    let ctx = test_context(&url_with_slash);
    let client = KasmClient::new(&ctx).unwrap();
    client.get_kasms().await.unwrap();

    mock.assert_async().await;
}

// ===================== get_images =====================

#[tokio::test]
async fn get_images_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_images")
        .with_status(200)
        .with_body(
            r#"{"images":[
                {"image_id":"img-001","friendly_name":"Ubuntu","enabled":true,"cores":2.0,"memory":2147483648},
                {"image_id":"img-002","friendly_name":"Kali Linux","enabled":false}
            ]}"#,
        )
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let images = client.get_images().await.unwrap();

    assert_eq!(images.len(), 2);
    assert_eq!(images[0].image_id, "img-001");
    assert_eq!(images[0].friendly_name.as_deref(), Some("Ubuntu"));
    assert_eq!(images[0].enabled, Some(true));
    assert_eq!(images[1].image_id, "img-002");
    assert_eq!(images[1].enabled, Some(false));

    mock.assert_async().await;
}

#[tokio::test]
async fn get_images_empty_list() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_images")
        .with_status(200)
        .with_body(r#"{"images":[]}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let images = client.get_images().await.unwrap();

    assert!(images.is_empty());

    mock.assert_async().await;
}

#[tokio::test]
async fn get_images_injects_auth_credentials() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/public/get_images")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"api_key":"test-key","api_key_secret":"test-secret"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"images":[]}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.get_images().await.unwrap();

    mock.assert_async().await;
}

// --- stop_kasm ---

#[tokio::test]
async fn stop_kasm_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/stop_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.stop_kasm("abc-123").await.unwrap();

    mock.assert_async().await;
}

// --- pause_kasm ---

#[tokio::test]
async fn pause_kasm_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/pause_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.pause_kasm("abc-123").await.unwrap();

    mock.assert_async().await;
}

// --- resume_kasm ---

#[tokio::test]
async fn resume_kasm_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api/resume_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.resume_kasm("abc-123").await.unwrap();

    mock.assert_async().await;
}

// --- URL construction ---

#[tokio::test]
async fn url_construction_without_trailing_slash() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(r#"{"kasms":[]}"#)
        .create_async()
        .await;

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.get_kasms().await.unwrap();

    mock.assert_async().await;
}
