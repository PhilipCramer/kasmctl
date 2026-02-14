use kasmctl::config::model::{Context, KasmConfig, NamedContext};
use kasmctl::config::{
    load_config_from, resolve_from_config, resolve_server_override, save_config_to,
};
use proptest::prelude::*;

fn arb_context() -> impl Strategy<Value = Context> {
    (
        "[a-zA-Z0-9._/-]{1,50}",
        "[a-zA-Z0-9]{1,32}",
        "[a-zA-Z0-9]{1,32}",
        any::<bool>(),
    )
        .prop_map(
            |(server, api_key, api_secret, insecure_skip_tls_verify)| Context {
                server,
                api_key,
                api_secret,
                insecure_skip_tls_verify,
            },
        )
}

fn arb_named_context() -> impl Strategy<Value = NamedContext> {
    ("[a-zA-Z0-9_-]{1,20}", arb_context())
        .prop_map(|(name, context)| NamedContext { name, context })
}

fn arb_config() -> impl Strategy<Value = KasmConfig> {
    prop::collection::vec(arb_named_context(), 0..5).prop_flat_map(|contexts| {
        let names: Vec<String> = contexts.iter().map(|c| c.name.clone()).collect();
        let current_strategy = if names.is_empty() {
            Just(None).boxed()
        } else {
            prop_oneof![Just(None), prop::sample::select(names).prop_map(Some),].boxed()
        };
        current_strategy.prop_map(move |current_context| KasmConfig {
            current_context,
            contexts: contexts.clone(),
        })
    })
}

// --- Property-based serde tests ---

proptest! {
    #[test]
    fn config_yaml_roundtrip(config in arb_config()) {
        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: KasmConfig = serde_yaml::from_str(&yaml).unwrap();
        prop_assert_eq!(config, deserialized);
    }

    #[test]
    fn insecure_flag_omitted_when_false(context in arb_context()) {
        let ctx = Context {
            insecure_skip_tls_verify: false,
            ..context
        };
        let yaml = serde_yaml::to_string(&ctx).unwrap();
        prop_assert!(
            !yaml.contains("insecure-skip-tls-verify"),
            "insecure flag should be omitted when false, got: {}",
            yaml
        );
    }

    #[test]
    fn insecure_flag_present_when_true(context in arb_context()) {
        let ctx = Context {
            insecure_skip_tls_verify: true,
            ..context
        };
        let yaml = serde_yaml::to_string(&ctx).unwrap();
        prop_assert!(
            yaml.contains("insecure-skip-tls-verify: true"),
            "insecure flag should be present when true, got: {}",
            yaml
        );
    }

    #[test]
    fn named_context_uses_serde_rename(nc in arb_named_context()) {
        let yaml = serde_yaml::to_string(&nc).unwrap();
        prop_assert!(
            yaml.contains("api-key:"),
            "expected 'api-key:' (serde rename), got: {}",
            yaml
        );
        prop_assert!(
            yaml.contains("api-secret:"),
            "expected 'api-secret:' (serde rename), got: {}",
            yaml
        );
        // flatten means no nested "context:" key
        prop_assert!(
            !yaml.contains("context:"),
            "expected flat structure, got nested 'context:' key: {}",
            yaml
        );
    }
}

// --- Property-based file I/O roundtrip ---

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]

    #[test]
    fn save_load_config_roundtrip(config in arb_config()) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.yaml");

        save_config_to(&path, &config).unwrap();
        let loaded = load_config_from(&path).unwrap();
        prop_assert_eq!(config, loaded);
    }
}

// --- Example-based tests ---

#[test]
fn default_config_is_empty() {
    let config = KasmConfig::default();
    assert!(config.current_context.is_none());
    assert!(config.contexts.is_empty());
}

#[test]
fn load_config_returns_default_when_file_missing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nonexistent.yaml");

    let config = load_config_from(&path).unwrap();
    assert_eq!(config, KasmConfig::default());
}

#[test]
fn save_config_creates_parent_directories() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("subdir").join("nested").join("config.yaml");

    let config = KasmConfig::default();
    save_config_to(&path, &config).unwrap();
    assert!(path.exists());
}

// --- resolve_server_override tests ---

#[test]
fn resolve_server_override_builds_context() {
    let ctx = resolve_server_override(
        "https://override.example.com",
        "env-key".into(),
        "env-secret".into(),
    )
    .unwrap();

    assert_eq!(ctx.server, "https://override.example.com");
    assert_eq!(ctx.api_key, "env-key");
    assert_eq!(ctx.api_secret, "env-secret");
    assert!(!ctx.insecure_skip_tls_verify);
}

// --- resolve_from_config tests ---

fn make_context(name: &str, server: &str) -> NamedContext {
    NamedContext {
        name: name.into(),
        context: Context {
            server: server.into(),
            api_key: format!("{name}-key"),
            api_secret: format!("{name}-secret"),
            insecure_skip_tls_verify: false,
        },
    }
}

#[test]
fn resolve_from_config_named_context_override() {
    let config = KasmConfig {
        current_context: Some("prod".into()),
        contexts: vec![
            make_context("prod", "https://prod.example.com"),
            make_context("staging", "https://staging.example.com"),
        ],
    };

    let ctx = resolve_from_config(&config, Some("staging")).unwrap();
    assert_eq!(ctx.server, "https://staging.example.com");
}

#[test]
fn resolve_from_config_uses_current_context() {
    let config = KasmConfig {
        current_context: Some("prod".into()),
        contexts: vec![
            make_context("prod", "https://prod.example.com"),
            make_context("staging", "https://staging.example.com"),
        ],
    };

    let ctx = resolve_from_config(&config, None).unwrap();
    assert_eq!(ctx.server, "https://prod.example.com");
}

#[test]
fn resolve_from_config_no_context_configured() {
    let config = KasmConfig {
        current_context: None,
        contexts: vec![],
    };

    let result = resolve_from_config(&config, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("no context"));
}

#[test]
fn resolve_from_config_current_context_not_found() {
    let config = KasmConfig {
        current_context: Some("missing".into()),
        contexts: vec![make_context("prod", "https://prod.example.com")],
    };

    let result = resolve_from_config(&config, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn resolve_from_config_override_not_found() {
    let config = KasmConfig {
        current_context: Some("prod".into()),
        contexts: vec![make_context("prod", "https://prod.example.com")],
    };

    let result = resolve_from_config(&config, Some("nonexistent"));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}
