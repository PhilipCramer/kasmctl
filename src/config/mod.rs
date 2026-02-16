pub mod model;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;

use self::model::{KasmConfig, NamedContext};

pub fn config_path() -> Result<PathBuf> {
    if let Ok(path) = std::env::var("KASMCTL_CONFIG") {
        return Ok(PathBuf::from(path));
    }
    let dirs =
        ProjectDirs::from("", "", "kasmctl").context("could not determine config directory")?;
    Ok(dirs.config_dir().join("config.yaml"))
}

pub fn load_config_from(path: &Path) -> Result<KasmConfig> {
    if !path.exists() {
        return Ok(KasmConfig::default());
    }
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    serde_yaml::from_str(&contents)
        .with_context(|| format!("failed to parse config file: {}", path.display()))
}

pub fn load_config() -> Result<KasmConfig> {
    load_config_from(&config_path()?)
}

pub fn save_config_to(path: &Path, config: &KasmConfig) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config directory: {}", parent.display()))?;
    }
    let yaml = serde_yaml::to_string(config).context("failed to serialize config")?;
    std::fs::write(path, yaml)
        .with_context(|| format!("failed to write config file: {}", path.display()))
}

pub fn save_config(config: &KasmConfig) -> Result<()> {
    save_config_to(&config_path()?, config)
}

/// Resolve the active context from CLI flags and config file.
/// Priority: --server flag > --context flag > current-context in config > error.
/// The `insecure` flag is applied on top of the resolved context when set.
pub fn resolve_context(
    server_override: Option<&str>,
    context_override: Option<&str>,
    insecure: bool,
) -> Result<model::Context> {
    let mut ctx = if let Some(server) = server_override {
        let api_key = std::env::var("KASMCTL_API_KEY")
            .context("--server requires KASMCTL_API_KEY environment variable")?;
        let api_secret = std::env::var("KASMCTL_API_SECRET")
            .context("--server requires KASMCTL_API_SECRET environment variable")?;
        resolve_server_override(server, api_key, api_secret)?
    } else {
        let config = load_config()?;
        resolve_from_config(&config, context_override)?
    };

    if insecure {
        ctx.insecure_skip_tls_verify = true;
    }

    Ok(ctx)
}

/// Build a context from an explicit server URL and credentials.
pub fn resolve_server_override(
    server: &str,
    api_key: String,
    api_secret: String,
) -> Result<model::Context> {
    Ok(model::Context {
        server: server.to_string(),
        api_key,
        api_secret,
        insecure_skip_tls_verify: false,
        timeout_seconds: None,
    })
}

/// Resolve a context from a loaded config, with an optional context name override.
pub fn resolve_from_config(
    config: &KasmConfig,
    context_override: Option<&str>,
) -> Result<model::Context> {
    let context_name = context_override
        .map(String::from)
        .or(config.current_context.clone())
        .context("no context configured — run `kasmctl config set-context` first")?;

    find_context(&config.contexts, &context_name)
}

fn find_context(contexts: &[NamedContext], name: &str) -> Result<model::Context> {
    contexts
        .iter()
        .find(|c| c.name == name)
        .map(|c| c.context.clone())
        .with_context(|| format!("context {name:?} not found in config"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model::Context;

    fn make_named(name: &str) -> NamedContext {
        NamedContext {
            name: name.into(),
            context: Context {
                server: format!("https://{name}.example.com"),
                api_key: "key".into(),
                api_secret: "secret".into(),
                insecure_skip_tls_verify: false,
                timeout_seconds: None,
            },
        }
    }

    #[test]
    fn find_context_success() {
        let contexts = vec![make_named("prod"), make_named("staging")];
        let ctx = find_context(&contexts, "staging").unwrap();
        assert_eq!(ctx.server, "https://staging.example.com");
    }

    #[test]
    fn find_context_not_found() {
        let contexts = vec![make_named("prod")];
        let result = find_context(&contexts, "missing");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn find_context_empty_list() {
        let result = find_context(&[], "anything");
        assert!(result.is_err());
    }
}
