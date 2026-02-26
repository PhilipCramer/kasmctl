use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use kasmctl::cli::config_cmd::ConfigCommand;
use kasmctl::cli::verbs::create::CreateResource;
use kasmctl::cli::verbs::delete::DeleteResource;
use kasmctl::cli::verbs::get::GetResource;
use kasmctl::cli::verbs::pause::PauseResource;
use kasmctl::cli::verbs::resume::ResumeResource;
use kasmctl::cli::verbs::stop::StopResource;
use kasmctl::cli::verbs::update::UpdateResource;
use kasmctl::cli::{Cli, Command};
use kasmctl::output::OutputFormat;

// --- Get commands ---

#[test]
fn parse_get_sessions() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { filters } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(filters.is_empty());
}

#[test]
fn parse_get_sessions_positional_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "get", "sessions", "abc-123"]);
    assert!(result.is_err());
}

// --- Get sessions --status filter ---

#[test]
fn parse_get_sessions_with_status_filter() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions", "--status", "running"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { filters } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.as_deref(), Some("running"));
}

#[test]
fn parse_get_sessions_without_status_defaults_to_none() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { filters } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(filters.status.is_none());
}

#[test]
fn parse_get_sessions_status_value_is_captured() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions", "--status", "stopped"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { filters } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.unwrap(), "stopped");
}

#[test]
fn parse_get_session_singular_alias() {
    let cli =
        Cli::try_parse_from(["kasmctl", "get", "session", "abc-123", "--user", "user-1"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Session { id, user } = args.resource else {
        panic!("expected Session resource");
    };
    assert_eq!(id, "abc-123");
    assert_eq!(user, "user-1");
}

#[test]
fn parse_get_kasm_alias() {
    let cli =
        Cli::try_parse_from(["kasmctl", "get", "kasm", "abc-123", "--user", "user-1"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Session { id, user } = args.resource else {
        panic!("expected Session resource");
    };
    assert_eq!(id, "abc-123");
    assert_eq!(user, "user-1");
}

#[test]
fn parse_get_kasms_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "kasms"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    assert!(matches!(args.resource, GetResource::Sessions { .. }));
}

#[test]
fn parse_get_session_requires_id() {
    let result = Cli::try_parse_from(["kasmctl", "get", "session"]);
    assert!(result.is_err());
}

// --- Get images ---

#[test]
fn parse_get_images() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "images"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { filters } = args.resource else {
        panic!("expected Images resource");
    };
    assert!(filters.is_empty());
}

#[test]
fn parse_get_image_singular_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "image", "img-abc"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Image { id } = args.resource else {
        panic!("expected Image resource");
    };
    assert_eq!(id, "img-abc");
}

#[test]
fn parse_get_images_positional_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "get", "images", "img-abc"]);
    assert!(result.is_err());
}

#[test]
fn parse_get_image_requires_id() {
    let result = Cli::try_parse_from(["kasmctl", "get", "image"]);
    assert!(result.is_err());
}

// --- Get images filter tests ---

#[test]
fn parse_get_images_with_enabled_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "images", "--enabled"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { filters } = args.resource else {
        panic!("expected Images resource");
    };
    assert!(filters.enabled);
    assert!(!filters.disabled);
    assert!(!filters.is_empty());
}

#[test]
fn parse_get_images_with_disabled_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "images", "--disabled"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { filters } = args.resource else {
        panic!("expected Images resource");
    };
    assert!(!filters.enabled);
    assert!(filters.disabled);
    assert!(!filters.is_empty());
}

#[test]
fn parse_get_images_enabled_disabled_conflict() {
    let result = Cli::try_parse_from(["kasmctl", "get", "images", "--enabled", "--disabled"]);
    assert!(result.is_err());
}

#[test]
fn parse_get_images_with_name_filter() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "images", "--name", "ubuntu"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { filters } = args.resource else {
        panic!("expected Images resource");
    };
    assert_eq!(filters.name.as_deref(), Some("ubuntu"));
    assert!(!filters.is_empty());
}

#[test]
fn parse_get_images_with_image_type_filter() {
    let cli =
        Cli::try_parse_from(["kasmctl", "get", "images", "--image-type", "Container"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { filters } = args.resource else {
        panic!("expected Images resource");
    };
    assert_eq!(filters.image_type.as_deref(), Some("Container"));
    assert!(!filters.is_empty());
}

#[test]
fn parse_get_images_with_all_filters() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "get",
        "images",
        "--enabled",
        "--name",
        "ubuntu",
        "--image-type",
        "Container",
    ])
    .unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { filters } = args.resource else {
        panic!("expected Images resource");
    };
    assert!(filters.enabled);
    assert!(!filters.disabled);
    assert_eq!(filters.name.as_deref(), Some("ubuntu"));
    assert_eq!(filters.image_type.as_deref(), Some("Container"));
    assert!(!filters.is_empty());
}

#[test]
fn parse_get_images_no_filters_is_empty() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "images"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { filters } = args.resource else {
        panic!("expected Images resource");
    };
    assert!(filters.is_empty());
    assert!(!filters.enabled);
    assert!(!filters.disabled);
    assert!(filters.name.is_none());
    assert!(filters.image_type.is_none());
}

// --- Get zones ---

#[test]
fn parse_get_zones() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "zones"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Zones { filters } = args.resource else {
        panic!("expected Zones resource");
    };
    assert!(filters.is_empty());
}

#[test]
fn parse_get_zone_by_id() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "zone", "zone-abc"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Zone { id } = args.resource else {
        panic!("expected Zone resource");
    };
    assert_eq!(id, "zone-abc");
}

#[test]
fn parse_get_zone_requires_id() {
    let result = Cli::try_parse_from(["kasmctl", "get", "zone"]);
    assert!(result.is_err());
}

#[test]
fn parse_get_zones_positional_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "get", "zones", "zone-abc"]);
    assert!(result.is_err());
}

#[test]
fn parse_get_zones_with_name_filter() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "zones", "--name", "us-east"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Zones { filters } = args.resource else {
        panic!("expected Zones resource");
    };
    assert_eq!(filters.name.as_deref(), Some("us-east"));
    assert!(!filters.is_empty());
}

// --- Create commands ---

#[test]
fn parse_create_session() {
    let cli = Cli::try_parse_from(["kasmctl", "create", "session", "--image", "img-123"]).unwrap();
    let Command::Create(args) = cli.command else {
        panic!("expected Create command");
    };
    let CreateResource::Session { image, user } = args.resource else {
        panic!("expected Session resource");
    };
    assert_eq!(image, "img-123");
    assert!(user.is_none());
}

#[test]
fn parse_create_session_with_user() {
    let cli = Cli::try_parse_from([
        "kasmctl", "create", "session", "--image", "img-123", "--user", "user-456",
    ])
    .unwrap();
    let Command::Create(args) = cli.command else {
        panic!("expected Create command");
    };
    let CreateResource::Session { image, user } = args.resource else {
        panic!("expected Session resource");
    };
    assert_eq!(image, "img-123");
    assert_eq!(user.as_deref(), Some("user-456"));
}

#[test]
fn parse_create_session_missing_image_fails() {
    let result = Cli::try_parse_from(["kasmctl", "create", "session"]);
    assert!(result.is_err());
}

#[test]
fn parse_create_kasm_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "create", "kasm", "--image", "img-123"]).unwrap();
    assert!(matches!(cli.command, Command::Create(_)));
}

// --- Create image commands ---

#[test]
fn parse_create_image_required_flags() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "create",
        "image",
        "--name",
        "kasmweb/terminal:1.18.0",
        "--friendly-name",
        "Terminal",
    ])
    .unwrap();
    let Command::Create(args) = cli.command else {
        panic!("expected Create command");
    };
    let CreateResource::Image {
        name,
        friendly_name,
        enabled,
        image_src,
        ..
    } = args.resource
    else {
        panic!("expected Image resource");
    };
    assert_eq!(name, "kasmweb/terminal:1.18.0");
    assert_eq!(friendly_name, "Terminal");
    assert!(enabled);
    assert_eq!(image_src, "Container");
}

#[test]
fn parse_create_image_all_flags() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "create",
        "image",
        "--name",
        "kasmweb/ubuntu:1.18.0",
        "--friendly-name",
        "Ubuntu Desktop",
        "--description",
        "Full Ubuntu desktop",
        "--cores",
        "4",
        "--memory",
        "4294967296",
        "--enabled",
        "false",
        "--image-src",
        "Server",
        "--docker-registry",
        "https://registry.example.com",
        "--run-config",
        r#"{"hostname":"test"}"#,
        "--exec-config",
        r#"{"go":{"cmd":"bash"}}"#,
        "--image-type",
        "Server",
    ])
    .unwrap();
    let Command::Create(args) = cli.command else {
        panic!("expected Create command");
    };
    let CreateResource::Image {
        name,
        friendly_name,
        description,
        cores,
        memory,
        enabled,
        image_src,
        docker_registry,
        run_config,
        exec_config,
        image_type,
    } = args.resource
    else {
        panic!("expected Image resource");
    };
    assert_eq!(name, "kasmweb/ubuntu:1.18.0");
    assert_eq!(friendly_name, "Ubuntu Desktop");
    assert_eq!(description.as_deref(), Some("Full Ubuntu desktop"));
    assert_eq!(cores, Some(4.0));
    assert_eq!(memory, Some(4294967296));
    assert!(!enabled);
    assert_eq!(image_src, "Server");
    assert_eq!(
        docker_registry.as_deref(),
        Some("https://registry.example.com")
    );
    assert_eq!(run_config.as_deref(), Some(r#"{"hostname":"test"}"#));
    assert_eq!(exec_config.as_deref(), Some(r#"{"go":{"cmd":"bash"}}"#));
    assert_eq!(image_type.as_deref(), Some("Server"));
}

#[test]
fn parse_create_image_missing_name_fails() {
    let result = Cli::try_parse_from(["kasmctl", "create", "image", "--friendly-name", "Terminal"]);
    assert!(result.is_err());
}

#[test]
fn parse_create_image_missing_friendly_name_fails() {
    let result = Cli::try_parse_from([
        "kasmctl",
        "create",
        "image",
        "--name",
        "kasmweb/terminal:1.18.0",
    ]);
    assert!(result.is_err());
}

#[test]
fn parse_create_image_enabled_defaults_to_true() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "create",
        "image",
        "--name",
        "img",
        "--friendly-name",
        "Img",
    ])
    .unwrap();
    let Command::Create(args) = cli.command else {
        panic!("expected Create command");
    };
    let CreateResource::Image { enabled, .. } = args.resource else {
        panic!("expected Image resource");
    };
    assert!(enabled);
}

#[test]
fn parse_create_image_enabled_can_be_set_false() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "create",
        "image",
        "--name",
        "img",
        "--friendly-name",
        "Img",
        "--enabled",
        "false",
    ])
    .unwrap();
    let Command::Create(args) = cli.command else {
        panic!("expected Create command");
    };
    let CreateResource::Image { enabled, .. } = args.resource else {
        panic!("expected Image resource");
    };
    assert!(!enabled);
}

#[test]
fn parse_create_image_src_defaults_to_container() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "create",
        "image",
        "--name",
        "img",
        "--friendly-name",
        "Img",
    ])
    .unwrap();
    let Command::Create(args) = cli.command else {
        panic!("expected Create command");
    };
    let CreateResource::Image { image_src, .. } = args.resource else {
        panic!("expected Image resource");
    };
    assert_eq!(image_src, "Container");
}

// --- Delete commands ---

#[test]
fn parse_delete_session() {
    let cli = Cli::try_parse_from(["kasmctl", "delete", "session", "kasm-789"]).unwrap();
    let Command::Delete(args) = cli.command else {
        panic!("expected Delete command");
    };
    let DeleteResource::Session { id } = args.resource else {
        panic!("expected Session resource");
    };
    assert_eq!(id, "kasm-789");
}

#[test]
fn parse_delete_session_missing_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "delete", "session"]);
    assert!(result.is_err());
}

#[test]
fn parse_delete_kasm_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "delete", "kasm", "kasm-789"]).unwrap();
    assert!(matches!(cli.command, Command::Delete(_)));
}

// --- Delete image commands ---

#[test]
fn parse_delete_image() {
    let cli = Cli::try_parse_from(["kasmctl", "delete", "image", "img-abc"]).unwrap();
    let Command::Delete(args) = cli.command else {
        panic!("expected Delete command");
    };
    let DeleteResource::Image { id } = args.resource else {
        panic!("expected Image resource");
    };
    assert_eq!(id, "img-abc");
}

#[test]
fn parse_delete_image_missing_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "delete", "image"]);
    assert!(result.is_err());
}

// --- Stop commands ---

#[test]
fn parse_stop_session() {
    let cli = Cli::try_parse_from(["kasmctl", "stop", "session", "kasm-789"]).unwrap();
    let Command::Stop(args) = cli.command else {
        panic!("expected Stop command");
    };
    let StopResource::Session { id } = args.resource else {
        panic!("expected Session resource");
    };
    assert_eq!(id, "kasm-789");
}

#[test]
fn parse_stop_kasm_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "stop", "kasm", "kasm-789"]).unwrap();
    assert!(matches!(cli.command, Command::Stop(_)));
}

#[test]
fn parse_stop_session_missing_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "stop", "session"]);
    assert!(result.is_err());
}

// --- Pause commands ---

#[test]
fn parse_pause_session() {
    let cli = Cli::try_parse_from(["kasmctl", "pause", "session", "kasm-789"]).unwrap();
    let Command::Pause(args) = cli.command else {
        panic!("expected Pause command");
    };
    let PauseResource::Session { id } = args.resource else {
        panic!("expected Session resource");
    };
    assert_eq!(id, "kasm-789");
}

#[test]
fn parse_pause_kasm_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "pause", "kasm", "kasm-789"]).unwrap();
    assert!(matches!(cli.command, Command::Pause(_)));
}

#[test]
fn parse_pause_session_missing_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "pause", "session"]);
    assert!(result.is_err());
}

// --- Resume commands ---

#[test]
fn parse_resume_session() {
    let cli = Cli::try_parse_from(["kasmctl", "resume", "session", "kasm-789"]).unwrap();
    let Command::Resume(args) = cli.command else {
        panic!("expected Resume command");
    };
    let ResumeResource::Session { id } = args.resource else {
        panic!("expected Session resource");
    };
    assert_eq!(id, "kasm-789");
}

#[test]
fn parse_resume_kasm_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "resume", "kasm", "kasm-789"]).unwrap();
    assert!(matches!(cli.command, Command::Resume(_)));
}

#[test]
fn parse_resume_session_missing_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "resume", "session"]);
    assert!(result.is_err());
}

// --- Config commands ---

#[test]
fn parse_config_set_context() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "config",
        "set-context",
        "prod",
        "--server",
        "https://kasm.example.com",
        "--api-key",
        "key123",
        "--api-secret",
        "secret456",
    ])
    .unwrap();
    let Command::Config(args) = cli.command else {
        panic!("expected Config command");
    };
    let ConfigCommand::SetContext {
        name,
        server,
        api_key,
        api_secret,
        insecure,
    } = args.command
    else {
        panic!("expected SetContext");
    };
    assert_eq!(name, "prod");
    assert_eq!(server, "https://kasm.example.com");
    assert_eq!(api_key, "key123");
    assert_eq!(api_secret, "secret456");
    assert!(!insecure);
}

#[test]
fn parse_config_set_context_with_insecure() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "config",
        "set-context",
        "dev",
        "--server",
        "https://dev.example.com",
        "--api-key",
        "key",
        "--api-secret",
        "secret",
        "--insecure",
        "true",
    ])
    .unwrap();
    let Command::Config(args) = cli.command else {
        panic!("expected Config command");
    };
    let ConfigCommand::SetContext { insecure, .. } = args.command else {
        panic!("expected SetContext");
    };
    assert!(insecure);
}

#[test]
fn parse_config_use_context() {
    let cli = Cli::try_parse_from(["kasmctl", "config", "use-context", "prod"]).unwrap();
    let Command::Config(args) = cli.command else {
        panic!("expected Config command");
    };
    assert!(matches!(
        args.command,
        ConfigCommand::UseContext { name } if name == "prod"
    ));
}

#[test]
fn parse_config_get_contexts() {
    let cli = Cli::try_parse_from(["kasmctl", "config", "get-contexts"]).unwrap();
    let Command::Config(args) = cli.command else {
        panic!("expected Config command");
    };
    assert!(matches!(args.command, ConfigCommand::GetContexts));
}

// --- Global flags ---

#[test]
fn parse_output_json() {
    let cli = Cli::try_parse_from(["kasmctl", "-o", "json", "get", "sessions"]).unwrap();
    assert!(matches!(cli.output, OutputFormat::Json));
}

#[test]
fn parse_output_yaml() {
    let cli = Cli::try_parse_from(["kasmctl", "--output", "yaml", "get", "sessions"]).unwrap();
    assert!(matches!(cli.output, OutputFormat::Yaml));
}

#[test]
fn parse_output_table_default() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions"]).unwrap();
    assert!(matches!(cli.output, OutputFormat::Table));
}

#[test]
fn parse_output_invalid_fails() {
    let result = Cli::try_parse_from(["kasmctl", "-o", "csv", "get", "sessions"]);
    assert!(result.is_err());
}

#[test]
fn parse_context_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "--context", "staging", "get", "sessions"]).unwrap();
    assert_eq!(cli.context.as_deref(), Some("staging"));
}

#[test]
fn parse_server_flag() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "--server",
        "https://custom.example.com",
        "get",
        "sessions",
    ])
    .unwrap();
    assert_eq!(cli.server.as_deref(), Some("https://custom.example.com"));
}

#[test]
fn parse_no_subcommand_fails() {
    let result = Cli::try_parse_from(["kasmctl"]);
    assert!(result.is_err());
}

// --- Get sessions filter tests ---

#[test]
fn parse_get_sessions_with_all_filters() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "get",
        "sessions",
        "--status",
        "running",
        "--image",
        "img-1",
        "--user",
        "user-1",
        "--host",
        "host-1",
        "--created-before",
        "2025-01-01 00:00:00",
        "--created-after",
        "2024-01-01 00:00:00",
        "--idle-since",
        "2025-06-01 00:00:00",
        "--idle-for",
        "2h",
    ])
    .unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { filters } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.as_deref(), Some("running"));
    assert_eq!(filters.image.as_deref(), Some("img-1"));
    assert_eq!(filters.user.as_deref(), Some("user-1"));
    assert_eq!(filters.host.as_deref(), Some("host-1"));
    assert_eq!(
        filters.created_before.as_deref(),
        Some("2025-01-01 00:00:00")
    );
    assert_eq!(
        filters.created_after.as_deref(),
        Some("2024-01-01 00:00:00")
    );
    assert_eq!(filters.idle_since.as_deref(), Some("2025-06-01 00:00:00"));
    assert_eq!(filters.idle_for.as_deref(), Some("2h"));
    assert!(!filters.is_empty());
}

// --- Stop sessions tests ---

#[test]
fn parse_stop_sessions_no_filters() {
    let cli = Cli::try_parse_from(["kasmctl", "stop", "sessions"]).unwrap();
    let Command::Stop(args) = cli.command else {
        panic!("expected Stop command");
    };
    let StopResource::Sessions { filters, yes } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(filters.is_empty());
    assert!(!yes);
}

#[test]
fn parse_stop_sessions_with_status() {
    let cli = Cli::try_parse_from(["kasmctl", "stop", "sessions", "--status", "running"]).unwrap();
    let Command::Stop(args) = cli.command else {
        panic!("expected Stop command");
    };
    let StopResource::Sessions { filters, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.as_deref(), Some("running"));
}

#[test]
fn parse_stop_sessions_all_filters() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "stop",
        "sessions",
        "--status",
        "running",
        "--image",
        "img-1",
        "--user",
        "user-1",
        "--host",
        "host-1",
        "--created-before",
        "2025-01-01 00:00:00",
        "--created-after",
        "2024-01-01 00:00:00",
        "--idle-since",
        "2025-06-01 00:00:00",
        "--idle-for",
        "1h30m",
    ])
    .unwrap();
    let Command::Stop(args) = cli.command else {
        panic!("expected Stop command");
    };
    let StopResource::Sessions { filters, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.as_deref(), Some("running"));
    assert_eq!(filters.image.as_deref(), Some("img-1"));
    assert_eq!(filters.user.as_deref(), Some("user-1"));
    assert_eq!(filters.host.as_deref(), Some("host-1"));
    assert_eq!(
        filters.created_before.as_deref(),
        Some("2025-01-01 00:00:00")
    );
    assert_eq!(
        filters.created_after.as_deref(),
        Some("2024-01-01 00:00:00")
    );
    assert_eq!(filters.idle_since.as_deref(), Some("2025-06-01 00:00:00"));
    assert_eq!(filters.idle_for.as_deref(), Some("1h30m"));
}

#[test]
fn parse_stop_sessions_yes_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "stop", "sessions", "--yes"]).unwrap();
    let Command::Stop(args) = cli.command else {
        panic!("expected Stop command");
    };
    let StopResource::Sessions { yes, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(yes);
}

#[test]
fn parse_stop_sessions_yes_short_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "stop", "sessions", "-y"]).unwrap();
    let Command::Stop(args) = cli.command else {
        panic!("expected Stop command");
    };
    let StopResource::Sessions { yes, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(yes);
}

#[test]
fn parse_stop_kasms_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "stop", "kasms"]).unwrap();
    let Command::Stop(args) = cli.command else {
        panic!("expected Stop command");
    };
    assert!(matches!(args.resource, StopResource::Sessions { .. }));
}

// --- Pause sessions tests ---

#[test]
fn parse_pause_sessions_no_filters() {
    let cli = Cli::try_parse_from(["kasmctl", "pause", "sessions"]).unwrap();
    let Command::Pause(args) = cli.command else {
        panic!("expected Pause command");
    };
    let PauseResource::Sessions { filters, yes } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(filters.is_empty());
    assert!(!yes);
}

#[test]
fn parse_pause_sessions_with_status() {
    let cli = Cli::try_parse_from(["kasmctl", "pause", "sessions", "--status", "running"]).unwrap();
    let Command::Pause(args) = cli.command else {
        panic!("expected Pause command");
    };
    let PauseResource::Sessions { filters, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.as_deref(), Some("running"));
}

#[test]
fn parse_pause_sessions_all_filters() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "pause",
        "sessions",
        "--status",
        "running",
        "--image",
        "img-1",
        "--user",
        "user-1",
        "--host",
        "host-1",
        "--created-before",
        "2025-01-01 00:00:00",
        "--created-after",
        "2024-01-01 00:00:00",
        "--idle-since",
        "2025-06-01 00:00:00",
        "--idle-for",
        "1h30m",
    ])
    .unwrap();
    let Command::Pause(args) = cli.command else {
        panic!("expected Pause command");
    };
    let PauseResource::Sessions { filters, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.as_deref(), Some("running"));
    assert_eq!(filters.image.as_deref(), Some("img-1"));
    assert_eq!(filters.user.as_deref(), Some("user-1"));
    assert_eq!(filters.host.as_deref(), Some("host-1"));
    assert_eq!(
        filters.created_before.as_deref(),
        Some("2025-01-01 00:00:00")
    );
    assert_eq!(
        filters.created_after.as_deref(),
        Some("2024-01-01 00:00:00")
    );
    assert_eq!(filters.idle_since.as_deref(), Some("2025-06-01 00:00:00"));
    assert_eq!(filters.idle_for.as_deref(), Some("1h30m"));
}

#[test]
fn parse_pause_sessions_yes_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "pause", "sessions", "--yes"]).unwrap();
    let Command::Pause(args) = cli.command else {
        panic!("expected Pause command");
    };
    let PauseResource::Sessions { yes, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(yes);
}

#[test]
fn parse_pause_sessions_yes_short_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "pause", "sessions", "-y"]).unwrap();
    let Command::Pause(args) = cli.command else {
        panic!("expected Pause command");
    };
    let PauseResource::Sessions { yes, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(yes);
}

#[test]
fn parse_pause_kasms_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "pause", "kasms"]).unwrap();
    let Command::Pause(args) = cli.command else {
        panic!("expected Pause command");
    };
    assert!(matches!(args.resource, PauseResource::Sessions { .. }));
}

// --- Resume sessions tests ---

#[test]
fn parse_resume_sessions_no_filters() {
    let cli = Cli::try_parse_from(["kasmctl", "resume", "sessions"]).unwrap();
    let Command::Resume(args) = cli.command else {
        panic!("expected Resume command");
    };
    let ResumeResource::Sessions { filters, yes } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(filters.is_empty());
    assert!(!yes);
}

#[test]
fn parse_resume_sessions_with_status() {
    let cli =
        Cli::try_parse_from(["kasmctl", "resume", "sessions", "--status", "stopped"]).unwrap();
    let Command::Resume(args) = cli.command else {
        panic!("expected Resume command");
    };
    let ResumeResource::Sessions { filters, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.as_deref(), Some("stopped"));
}

#[test]
fn parse_resume_sessions_all_filters() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "resume",
        "sessions",
        "--status",
        "stopped",
        "--image",
        "img-1",
        "--user",
        "user-1",
        "--host",
        "host-1",
        "--created-before",
        "2025-01-01 00:00:00",
        "--created-after",
        "2024-01-01 00:00:00",
        "--idle-since",
        "2025-06-01 00:00:00",
        "--idle-for",
        "1h30m",
    ])
    .unwrap();
    let Command::Resume(args) = cli.command else {
        panic!("expected Resume command");
    };
    let ResumeResource::Sessions { filters, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(filters.status.as_deref(), Some("stopped"));
    assert_eq!(filters.image.as_deref(), Some("img-1"));
    assert_eq!(filters.user.as_deref(), Some("user-1"));
    assert_eq!(filters.host.as_deref(), Some("host-1"));
    assert_eq!(
        filters.created_before.as_deref(),
        Some("2025-01-01 00:00:00")
    );
    assert_eq!(
        filters.created_after.as_deref(),
        Some("2024-01-01 00:00:00")
    );
    assert_eq!(filters.idle_since.as_deref(), Some("2025-06-01 00:00:00"));
    assert_eq!(filters.idle_for.as_deref(), Some("1h30m"));
}

#[test]
fn parse_resume_sessions_yes_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "resume", "sessions", "--yes"]).unwrap();
    let Command::Resume(args) = cli.command else {
        panic!("expected Resume command");
    };
    let ResumeResource::Sessions { yes, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(yes);
}

#[test]
fn parse_resume_sessions_yes_short_flag() {
    let cli = Cli::try_parse_from(["kasmctl", "resume", "sessions", "-y"]).unwrap();
    let Command::Resume(args) = cli.command else {
        panic!("expected Resume command");
    };
    let ResumeResource::Sessions { yes, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(yes);
}

#[test]
fn parse_resume_kasms_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "resume", "kasms"]).unwrap();
    let Command::Resume(args) = cli.command else {
        panic!("expected Resume command");
    };
    assert!(matches!(args.resource, ResumeResource::Sessions { .. }));
}

// --- Completion commands ---

#[test]
fn parse_completion_bash() {
    let cli = Cli::try_parse_from(["kasmctl", "completion", "bash"]).unwrap();
    assert!(matches!(
        cli.command,
        Command::Completion { shell: Shell::Bash }
    ));
}

#[test]
fn parse_completion_zsh() {
    let cli = Cli::try_parse_from(["kasmctl", "completion", "zsh"]).unwrap();
    assert!(matches!(
        cli.command,
        Command::Completion { shell: Shell::Zsh }
    ));
}

#[test]
fn parse_completion_fish() {
    let cli = Cli::try_parse_from(["kasmctl", "completion", "fish"]).unwrap();
    assert!(matches!(
        cli.command,
        Command::Completion { shell: Shell::Fish }
    ));
}

#[test]
fn parse_completion_powershell() {
    let cli = Cli::try_parse_from(["kasmctl", "completion", "powershell"]).unwrap();
    assert!(matches!(
        cli.command,
        Command::Completion {
            shell: Shell::PowerShell
        }
    ));
}

#[test]
fn parse_completion_elvish() {
    let cli = Cli::try_parse_from(["kasmctl", "completion", "elvish"]).unwrap();
    assert!(matches!(
        cli.command,
        Command::Completion {
            shell: Shell::Elvish
        }
    ));
}

#[test]
fn parse_completion_missing_shell_fails() {
    let result = Cli::try_parse_from(["kasmctl", "completion"]);
    assert!(result.is_err());
}

#[test]
fn parse_completion_invalid_shell_fails() {
    let result = Cli::try_parse_from(["kasmctl", "completion", "nushell"]);
    assert!(result.is_err());
}

#[test]
fn completion_generates_nonempty_output_for_each_shell() {
    use clap_complete::generate;

    for shell in [
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Elvish,
    ] {
        let mut buf = Vec::new();
        let mut cmd = Cli::command();
        generate(shell, &mut cmd, "kasmctl", &mut buf);
        assert!(
            !buf.is_empty(),
            "completion output for {shell:?} should not be empty"
        );
        let output = String::from_utf8(buf).expect("completion output should be valid UTF-8");
        assert!(
            output.contains("kasmctl"),
            "completion output for {shell:?} should reference the binary name"
        );
    }
}

// --- Update commands ---

#[test]
fn parse_update_image_with_id_only() {
    let cli = Cli::try_parse_from(["kasmctl", "update", "image", "img-abc"]).unwrap();
    let Command::Update(args) = cli.command else {
        panic!("expected Update command");
    };
    let UpdateResource::Image { id, .. } = args.resource;
    assert_eq!(id, "img-abc");
}

#[test]
fn parse_update_image_missing_id_fails() {
    let result = Cli::try_parse_from(["kasmctl", "update", "image"]);
    assert!(result.is_err());
}

#[test]
fn parse_update_image_with_all_flags() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "update",
        "image",
        "img-abc",
        "--name",
        "kasmweb/ubuntu:1.18.0",
        "--friendly-name",
        "Ubuntu Desktop",
        "--description",
        "A desktop workspace",
        "--cores",
        "4.0",
        "--memory",
        "4096000000",
        "--enabled",
        "true",
        "--image-src",
        "img/thumbnails/ubuntu.png",
        "--docker-registry",
        "https://index.docker.io/v1/",
        "--run-config",
        "{}",
        "--exec-config",
        "{}",
        "--hidden",
        "false",
    ])
    .unwrap();
    let Command::Update(args) = cli.command else {
        panic!("expected Update command");
    };
    let UpdateResource::Image {
        id,
        name,
        friendly_name,
        description,
        cores,
        memory,
        enabled,
        image_src,
        docker_registry,
        run_config,
        exec_config,
        hidden,
    } = args.resource;
    assert_eq!(id, "img-abc");
    assert_eq!(name.as_deref(), Some("kasmweb/ubuntu:1.18.0"));
    assert_eq!(friendly_name.as_deref(), Some("Ubuntu Desktop"));
    assert_eq!(description.as_deref(), Some("A desktop workspace"));
    assert_eq!(cores, Some(4.0));
    assert_eq!(memory, Some(4096000000));
    assert_eq!(enabled, Some(true));
    assert_eq!(image_src.as_deref(), Some("img/thumbnails/ubuntu.png"));
    assert_eq!(
        docker_registry.as_deref(),
        Some("https://index.docker.io/v1/")
    );
    assert_eq!(run_config.as_deref(), Some("{}"));
    assert_eq!(exec_config.as_deref(), Some("{}"));
    assert_eq!(hidden, Some(false));
}

#[test]
fn parse_update_image_with_partial_flags() {
    let cli = Cli::try_parse_from([
        "kasmctl",
        "update",
        "image",
        "img-abc",
        "--friendly-name",
        "New Name",
        "--enabled",
        "false",
    ])
    .unwrap();
    let Command::Update(args) = cli.command else {
        panic!("expected Update command");
    };
    let UpdateResource::Image {
        id,
        name,
        friendly_name,
        enabled,
        ..
    } = args.resource;
    assert_eq!(id, "img-abc");
    assert!(name.is_none());
    assert_eq!(friendly_name.as_deref(), Some("New Name"));
    assert_eq!(enabled, Some(false));
}

#[test]
fn parse_update_no_subcommand_fails() {
    let result = Cli::try_parse_from(["kasmctl", "update"]);
    assert!(result.is_err());
}
