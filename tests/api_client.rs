use kasmctl::api::KasmClient;
use kasmctl::config::model::Context;

fn test_context(server_url: &str) -> Context {
    Context {
        server: server_url.to_string(),
        api_key: "test-key".into(),
        api_secret: "test-secret".into(),
        insecure_skip_tls_verify: false,
        timeout_seconds: None,
    }
}

// --- Auth injection ---

#[test]
fn post_injects_api_credentials() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"api_key":"test-key","api_key_secret":"test-secret"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"kasms":[]}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.get_kasms().unwrap();

    mock.assert();
}

// --- get_kasms ---

#[test]
fn get_kasms_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(
            r#"{"kasms":[
                {"kasm_id":"abc-123","operational_status":"running"},
                {"kasm_id":"def-456","operational_status":"stopped"}
            ]}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let sessions = client.get_kasms().unwrap();

    assert_eq!(sessions.len(), 2);
    assert_eq!(sessions[0].kasm_id, "abc-123");
    assert_eq!(sessions[1].kasm_id, "def-456");

    mock.assert();
}

#[test]
fn get_kasms_deserializes_nested_image() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(
            r#"{"kasms":[{
                "kasm_id":"abc-123",
                "operational_status":"running",
                "image_id":"img-001",
                "image": {
                    "friendly_name": "Ubuntu Desktop",
                    "name": "kasmweb/ubuntu-focal:1.14.0"
                }
            }]}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let sessions = client.get_kasms().unwrap();

    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].kasm_id, "abc-123");
    let img = sessions[0].image.as_ref().expect("image should be present");
    assert_eq!(img.friendly_name.as_deref(), Some("Ubuntu Desktop"));
    assert_eq!(img.name.as_deref(), Some("kasmweb/ubuntu-focal:1.14.0"));

    mock.assert();
}

#[test]
fn get_kasms_empty_list() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(r#"{"kasms":[]}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let sessions = client.get_kasms().unwrap();

    assert!(sessions.is_empty());

    mock.assert();
}

// --- get_kasm_status ---

#[test]
fn get_kasm_status_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_kasm_status")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123","user_id":"user-456"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"kasm":{"kasm_id":"abc-123","operational_status":"running"}}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let session = client.get_kasm_status("abc-123", "user-456").unwrap();

    assert_eq!(session.kasm_id, "abc-123");
    assert_eq!(session.operational_status.as_deref(), Some("running"));

    mock.assert();
}

// --- request_kasm ---

#[test]
fn request_kasm_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/request_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"image_id":"img-001"}"#.into(),
        ))
        .with_status(200)
        .with_body(
            r#"{"kasm_id":"new-kasm","status":"starting","kasm_url":"https://kasm.example.com/session"}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let resp = client.request_kasm("img-001", None).unwrap();

    assert_eq!(resp.kasm_id, "new-kasm");
    assert_eq!(resp.status.as_deref(), Some("starting"));
    assert_eq!(
        resp.kasm_url.as_deref(),
        Some("https://kasm.example.com/session")
    );

    mock.assert();
}

#[test]
fn request_kasm_with_user_id() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/request_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"image_id":"img-001","user_id":"user-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"kasm_id":"new-kasm"}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.request_kasm("img-001", Some("user-123")).unwrap();

    mock.assert();
}

#[test]
fn request_kasm_without_user_id_omits_field() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/request_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"image_id":"img-001"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"kasm_id":"new-kasm"}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.request_kasm("img-001", None).unwrap();

    mock.assert();
}

// --- destroy_kasm ---

#[test]
fn destroy_kasm_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/destroy_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.destroy_kasm("abc-123").unwrap();

    mock.assert();
}

// --- Error handling ---

#[test]
fn api_error_message_in_response() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(r#"{"error_message":"unauthorized"}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let result = client.get_kasms();

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("unauthorized"), "error was: {err}");

    mock.assert();
}

#[test]
fn api_http_error_status() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(500)
        .with_body(r#"{"something":"else"}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let result = client.get_kasms();

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("500"), "error was: {err}");

    mock.assert();
}

#[test]
fn api_deserialization_error() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body("not json at all")
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let result = client.get_kasms();

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("parse") || err.contains("expected"),
        "error was: {err}"
    );

    mock.assert();
}

// --- URL construction ---

#[test]
fn url_construction_with_trailing_slash() {
    let mut server = mockito::Server::new();
    let url_with_slash = format!("{}/", server.url());

    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(r#"{"kasms":[]}"#)
        .create();

    let ctx = test_context(&url_with_slash);
    let client = KasmClient::new(&ctx).unwrap();
    client.get_kasms().unwrap();

    mock.assert();
}

// ===================== get_images =====================

#[test]
fn get_images_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_images")
        .with_status(200)
        .with_body(
            r#"{"images":[
                {"image_id":"img-001","friendly_name":"Ubuntu","enabled":true,"cores":2.0,"memory":2147483648},
                {"image_id":"img-002","friendly_name":"Kali Linux","enabled":false}
            ]}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let images = client.get_images().unwrap();

    assert_eq!(images.len(), 2);
    assert_eq!(images[0].image_id, "img-001");
    assert_eq!(images[0].friendly_name.as_deref(), Some("Ubuntu"));
    assert_eq!(images[0].enabled, Some(true));
    assert_eq!(images[1].image_id, "img-002");
    assert_eq!(images[1].enabled, Some(false));

    mock.assert();
}

#[test]
fn get_images_empty_list() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_images")
        .with_status(200)
        .with_body(r#"{"images":[]}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let images = client.get_images().unwrap();

    assert!(images.is_empty());

    mock.assert();
}

#[test]
fn get_images_injects_auth_credentials() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_images")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"api_key":"test-key","api_key_secret":"test-secret"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"images":[]}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.get_images().unwrap();

    mock.assert();
}

// --- create_image ---

#[test]
fn create_image_success() {
    use kasmctl::api::images::CreateImageParams;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/create_image")
        .with_status(200)
        .with_body(
            r#"{"image":{"image_id":"new-img-1","friendly_name":"Terminal","name":"kasmweb/terminal:1.18.0","description":null,"enabled":true,"cores":2.0,"memory":2147483648,"image_src":"Container"}}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let params = CreateImageParams {
        name: "kasmweb/terminal:1.18.0".into(),
        friendly_name: "Terminal".into(),
        description: None,
        cores: Some(2.0),
        memory: Some(2147483648),
        enabled: true,
        image_src: "Container".into(),
        docker_registry: None,
        run_config: None,
        exec_config: None,
        image_type: None,
    };
    let image = client.create_image(&params).unwrap();

    assert_eq!(image.image_id, "new-img-1");
    assert_eq!(image.friendly_name.as_deref(), Some("Terminal"));
    mock.assert();
}

#[test]
fn create_image_sends_target_image_wrapper() {
    use kasmctl::api::images::CreateImageParams;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/create_image")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"target_image":{"name":"kasmweb/terminal:1.18.0","friendly_name":"Terminal","enabled":true,"image_src":"Container"}}"#.into(),
        ))
        .with_status(200)
        .with_body(
            r#"{"image":{"image_id":"new-img-1","friendly_name":"Terminal","name":"kasmweb/terminal:1.18.0","description":null,"enabled":true,"cores":2.0,"memory":2147483648,"image_src":"Container"}}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let params = CreateImageParams {
        name: "kasmweb/terminal:1.18.0".into(),
        friendly_name: "Terminal".into(),
        description: None,
        cores: None,
        memory: None,
        enabled: true,
        image_src: "Container".into(),
        docker_registry: None,
        run_config: None,
        exec_config: None,
        image_type: None,
    };
    client.create_image(&params).unwrap();

    mock.assert();
}

#[test]
fn create_image_injects_auth_credentials() {
    use kasmctl::api::images::CreateImageParams;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/create_image")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"api_key":"test-key","api_key_secret":"test-secret"}"#.into(),
        ))
        .with_status(200)
        .with_body(
            r#"{"image":{"image_id":"new-img-1","friendly_name":"Terminal","name":"kasmweb/terminal:1.18.0","description":null,"enabled":true,"cores":2.0,"memory":2147483648,"image_src":"Container"}}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let params = CreateImageParams {
        name: "kasmweb/terminal:1.18.0".into(),
        friendly_name: "Terminal".into(),
        description: None,
        cores: None,
        memory: None,
        enabled: true,
        image_src: "Container".into(),
        docker_registry: None,
        run_config: None,
        exec_config: None,
        image_type: None,
    };
    client.create_image(&params).unwrap();

    mock.assert();
}

#[test]
fn create_image_api_error() {
    use kasmctl::api::images::CreateImageParams;

    let mut server = mockito::Server::new();
    let _mock = server
        .mock("POST", "/api/admin/create_image")
        .with_status(200)
        .with_body(r#"{"error_message":"image already exists"}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let params = CreateImageParams {
        name: "kasmweb/terminal:1.18.0".into(),
        friendly_name: "Terminal".into(),
        description: None,
        cores: None,
        memory: None,
        enabled: true,
        image_src: "Container".into(),
        docker_registry: None,
        run_config: None,
        exec_config: None,
        image_type: None,
    };
    let result = client.create_image(&params);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("image already exists"), "error was: {err}");
}

// --- delete_image ---

#[test]
fn delete_image_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/delete_image")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"target_image":{"image_id":"img-abc"}}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.delete_image("img-abc").unwrap();

    mock.assert();
}

#[test]
fn delete_image_injects_auth_credentials() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/delete_image")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"api_key":"test-key","api_key_secret":"test-secret"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.delete_image("img-abc").unwrap();

    mock.assert();
}

#[test]
fn delete_image_api_error() {
    let mut server = mockito::Server::new();
    let _mock = server
        .mock("POST", "/api/admin/delete_image")
        .with_status(200)
        .with_body(r#"{"error_message":"image not found"}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let result = client.delete_image("img-abc");

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("image not found"), "error was: {err}");
}

// --- update_image ---

#[test]
fn update_image_success() {
    use kasmctl::api::images::UpdateImageRequest;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/update_image")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"target_image":{"image_id":"img-001","friendly_name":"New Name"}}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"image":{"image_id":"img-001","friendly_name":"New Name","enabled":true}}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let req = UpdateImageRequest {
        image_id: "img-001".into(),
        name: None,
        friendly_name: Some("New Name".into()),
        description: None,
        cores: None,
        memory: None,
        enabled: None,
        image_src: None,
        docker_registry: None,
        run_config: None,
        exec_config: None,
        hidden: None,
    };
    let image = client.update_image(&req).unwrap();

    assert_eq!(image.image_id, "img-001");
    assert_eq!(image.friendly_name.as_deref(), Some("New Name"));
    mock.assert();
}

#[test]
fn update_image_sends_target_image_wrapper() {
    use kasmctl::api::images::UpdateImageRequest;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/update_image")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"target_image":{"image_id":"img-001","cores":4.0,"enabled":false}}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"image":{"image_id":"img-001"}}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let req = UpdateImageRequest {
        image_id: "img-001".into(),
        name: None,
        friendly_name: None,
        description: None,
        cores: Some(4.0),
        memory: None,
        enabled: Some(false),
        image_src: None,
        docker_registry: None,
        run_config: None,
        exec_config: None,
        hidden: None,
    };
    client.update_image(&req).unwrap();

    mock.assert();
}

#[test]
fn update_image_omits_none_fields() {
    use kasmctl::api::images::UpdateImageRequest;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/update_image")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"target_image":{"image_id":"img-001"}}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"image":{"image_id":"img-001"}}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let req = UpdateImageRequest {
        image_id: "img-001".into(),
        name: None,
        friendly_name: None,
        description: None,
        cores: None,
        memory: None,
        enabled: None,
        image_src: None,
        docker_registry: None,
        run_config: None,
        exec_config: None,
        hidden: None,
    };
    client.update_image(&req).unwrap();

    mock.assert();
}

#[test]
fn update_image_injects_auth_credentials() {
    use kasmctl::api::images::UpdateImageRequest;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/update_image")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"api_key":"test-key","api_key_secret":"test-secret"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"image":{"image_id":"img-001"}}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let req = UpdateImageRequest {
        image_id: "img-001".into(),
        name: None,
        friendly_name: None,
        description: None,
        cores: None,
        memory: None,
        enabled: None,
        image_src: None,
        docker_registry: None,
        run_config: None,
        exec_config: None,
        hidden: None,
    };
    client.update_image(&req).unwrap();

    mock.assert();
}

#[test]
fn update_image_api_error() {
    use kasmctl::api::images::UpdateImageRequest;

    let mut server = mockito::Server::new();
    let _mock = server
        .mock("POST", "/api/admin/update_image")
        .with_status(200)
        .with_body(r#"{"error_message":"image not found"}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let req = UpdateImageRequest {
        image_id: "nonexistent".into(),
        name: None,
        friendly_name: None,
        description: None,
        cores: None,
        memory: None,
        enabled: None,
        image_src: None,
        docker_registry: None,
        run_config: None,
        exec_config: None,
        hidden: None,
    };
    let result = client.update_image(&req);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("image not found"), "error was: {err}");
}

// --- stop_kasm ---

#[test]
fn stop_kasm_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/stop_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.stop_kasm("abc-123").unwrap();

    mock.assert();
}

// --- pause_kasm ---

#[test]
fn pause_kasm_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/pause_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.pause_kasm("abc-123").unwrap();

    mock.assert();
}

// --- resume_kasm ---

#[test]
fn resume_kasm_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/resume_kasm")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"kasm_id":"abc-123"}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.resume_kasm("abc-123").unwrap();

    mock.assert();
}

// --- get_zones ---

#[test]
fn get_zones_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_zones")
        .with_status(200)
        .with_body(
            r#"{"zones":[
                {"zone_id":"zone-001","zone_name":"us-east","load_balancing_strategy":"round_robin","proxy_connections":true},
                {"zone_id":"zone-002","zone_name":"eu-west"}
            ]}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let zones = client.get_zones().unwrap();

    assert_eq!(zones.len(), 2);
    assert_eq!(zones[0].zone_id, "zone-001");
    assert_eq!(zones[0].zone_name.as_deref(), Some("us-east"));
    assert_eq!(
        zones[0].load_balancing_strategy.as_deref(),
        Some("round_robin")
    );
    assert_eq!(zones[0].proxy_connections, Some(true));
    assert_eq!(zones[1].zone_id, "zone-002");
    assert_eq!(zones[1].zone_name.as_deref(), Some("eu-west"));

    mock.assert();
}

#[test]
fn get_zones_empty_list() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/public/get_zones")
        .with_status(200)
        .with_body(r#"{"zones":[]}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let zones = client.get_zones().unwrap();

    assert!(zones.is_empty());

    mock.assert();
}

// ===================== get_agents =====================

#[test]
fn get_agents_success() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/get_agents")
        .with_status(200)
        .with_body(
            r#"{"agents":[
                {"agent_id":"agent-001","hostname":"kasm-agent-1","status":"running","enabled":true},
                {"agent_id":"agent-002","hostname":"kasm-agent-2","status":"stopped","enabled":false}
            ]}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let agents = client.get_agents().unwrap();

    assert_eq!(agents.len(), 2);
    assert_eq!(agents[0].agent_id, "agent-001");
    assert_eq!(agents[0].hostname.as_deref(), Some("kasm-agent-1"));
    assert_eq!(agents[0].enabled, Some(true));
    assert_eq!(agents[1].agent_id, "agent-002");
    assert_eq!(agents[1].enabled, Some(false));

    mock.assert();
}

#[test]
fn get_agents_empty_list() {
    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/get_agents")
        .with_status(200)
        .with_body(r#"{"agents":[]}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let agents = client.get_agents().unwrap();

    assert!(agents.is_empty());

    mock.assert();
}

// ===================== update_agent =====================

#[test]
fn update_agent_success() {
    use kasmctl::api::agents::UpdateAgentRequest;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/update_agent")
        .with_status(200)
        .with_body(
            r#"{"agent":{"agent_id":"agent-001","hostname":"kasm-agent-1","enabled":false}}"#,
        )
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let req = UpdateAgentRequest {
        agent_id: "agent-001".into(),
        enabled: Some(false),
        cores_override: None,
        memory_override: None,
        gpus_override: None,
        auto_prune_images: None,
    };
    let agent = client.update_agent(&req).unwrap();

    assert_eq!(agent.agent_id, "agent-001");
    assert_eq!(agent.enabled, Some(false));

    mock.assert();
}

#[test]
fn update_agent_sends_target_agent_wrapper() {
    use kasmctl::api::agents::UpdateAgentRequest;

    let mut server = mockito::Server::new();
    let mock = server
        .mock("POST", "/api/admin/update_agent")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"target_agent":{"agent_id":"agent-001"}}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"agent":{"agent_id":"agent-001"}}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let req = UpdateAgentRequest {
        agent_id: "agent-001".into(),
        enabled: None,
        cores_override: None,
        memory_override: None,
        gpus_override: None,
        auto_prune_images: None,
    };
    client.update_agent(&req).unwrap();

    mock.assert();
}

#[test]
fn update_agent_omits_none_fields() {
    use kasmctl::api::agents::UpdateAgentRequest;

    let mut server = mockito::Server::new();
    // Only agent_id should be in target_agent — no enabled, cores_override, etc.
    let mock = server
        .mock("POST", "/api/admin/update_agent")
        .match_body(mockito::Matcher::PartialJsonString(
            r#"{"target_agent":{"agent_id":"agent-001"}}"#.into(),
        ))
        .with_status(200)
        .with_body(r#"{"agent":{"agent_id":"agent-001"}}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    let req = UpdateAgentRequest {
        agent_id: "agent-001".into(),
        enabled: None,
        cores_override: None,
        memory_override: None,
        gpus_override: None,
        auto_prune_images: None,
    };
    client.update_agent(&req).unwrap();

    mock.assert();
}

// --- URL construction ---

#[test]
fn url_construction_without_trailing_slash() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/api/public/get_kasms")
        .with_status(200)
        .with_body(r#"{"kasms":[]}"#)
        .create();

    let ctx = test_context(&server.url());
    let client = KasmClient::new(&ctx).unwrap();
    client.get_kasms().unwrap();

    mock.assert();
}
