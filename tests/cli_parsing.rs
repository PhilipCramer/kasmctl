use clap::Parser;
use kasmctl::cli::config_cmd::ConfigCommand;
use kasmctl::cli::verbs::create::CreateResource;
use kasmctl::cli::verbs::delete::DeleteResource;
use kasmctl::cli::verbs::get::GetResource;
use kasmctl::cli::{Cli, Command};
use kasmctl::output::OutputFormat;

// --- Get commands ---

#[test]
fn parse_get_sessions() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { id, status } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(id.is_none());
    assert!(status.is_none());
}

#[test]
fn parse_get_sessions_with_id() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions", "abc-123"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { id, status } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(id.as_deref(), Some("abc-123"));
    assert!(status.is_none());
}

// --- Get sessions --status filter ---

#[test]
fn parse_get_sessions_with_status_filter() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions", "--status", "running"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { id, status } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(id.is_none());
    assert_eq!(status.as_deref(), Some("running"));
}

#[test]
fn parse_get_sessions_without_status_defaults_to_none() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { status, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert!(status.is_none());
}

#[test]
fn parse_get_sessions_status_value_is_captured() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "sessions", "--status", "stopped"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Sessions { status, .. } = args.resource else {
        panic!("expected Sessions resource");
    };
    assert_eq!(status.unwrap(), "stopped");
}

#[test]
fn parse_get_session_singular_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "session"]).unwrap();
    assert!(matches!(cli.command, Command::Get(_)));
}

#[test]
fn parse_get_kasm_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "kasm"]).unwrap();
    assert!(matches!(cli.command, Command::Get(_)));
}

#[test]
fn parse_get_kasms_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "kasms"]).unwrap();
    assert!(matches!(cli.command, Command::Get(_)));
}

// --- Get images ---

#[test]
fn parse_get_images() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "images"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { id } = args.resource else {
        panic!("expected Images resource");
    };
    assert!(id.is_none());
}

#[test]
fn parse_get_image_singular_alias() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "image"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    assert!(matches!(args.resource, GetResource::Images { .. }));
}

#[test]
fn parse_get_images_with_id() {
    let cli = Cli::try_parse_from(["kasmctl", "get", "images", "img-abc"]).unwrap();
    let Command::Get(args) = cli.command else {
        panic!("expected Get command");
    };
    let GetResource::Images { id } = args.resource else {
        panic!("expected Images resource");
    };
    assert_eq!(id.as_deref(), Some("img-abc"));
}

// --- Create commands ---

#[test]
fn parse_create_session() {
    let cli = Cli::try_parse_from(["kasmctl", "create", "session", "--image", "img-123"]).unwrap();
    let Command::Create(args) = cli.command else {
        panic!("expected Create command");
    };
    let CreateResource::Session { image, user } = args.resource;
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
    let CreateResource::Session { image, user } = args.resource;
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

// --- Delete commands ---

#[test]
fn parse_delete_session() {
    let cli = Cli::try_parse_from(["kasmctl", "delete", "session", "kasm-789"]).unwrap();
    let Command::Delete(args) = cli.command else {
        panic!("expected Delete command");
    };
    let DeleteResource::Session { id } = args.resource;
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
