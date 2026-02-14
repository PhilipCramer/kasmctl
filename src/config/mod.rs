pub mod model;

use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;

use self::model::{KasmConfig, NamedContext};

pub fn config_path() -> Result<PathBuf> {
    if let Ok(path) = std::env::var("KASMCTL_CONFIG") {
        return Ok(PathBuf::from(path));
    }
    let dirs = ProjectDirs::from("", "", "kasmctl")
        .context("could not determine config directory")?;
    Ok(dirs.config_dir().join("config.yaml"))
}

pub fn load_config() -> Result<KasmConfig> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(KasmConfig::default());
    }
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    serde_yaml::from_str(&contents)
        .with_context(|| format!("failed to parse config file: {}", path.display()))
}

pub fn save_config(config: &KasmConfig) -> Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config directory: {}", parent.display()))?;
    }
    let yaml = serde_yaml::to_string(config).context("failed to serialize config")?;
    std::fs::write(&path, yaml)
        .with_context(|| format!("failed to write config file: {}", path.display()))
}

/// Resolve the active context from CLI flags and config file.
/// Priority: --server flag > --context flag > current-context in config > error.
pub fn resolve_context(
    server_override: Option<&str>,
    context_override: Option<&str>,
) -> Result<model::Context> {
    // Direct server override via CLI flags — requires env vars for keys
    if let Some(server) = server_override {
        let api_key = std::env::var("KASMCTL_API_KEY")
            .context("--server requires KASMCTL_API_KEY environment variable")?;
        let api_secret = std::env::var("KASMCTL_API_SECRET")
            .context("--server requires KASMCTL_API_SECRET environment variable")?;
        return Ok(model::Context {
            server: server.to_string(),
            api_key,
            api_secret,
            insecure_skip_tls_verify: false,
        });
    }

    let config = load_config()?;

    let context_name = context_override
        .map(String::from)
        .or(config.current_context)
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
