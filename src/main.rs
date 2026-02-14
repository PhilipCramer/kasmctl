use anyhow::{Context, Result};
use clap::Parser;
use comfy_table::{Table, presets::UTF8_FULL_CONDENSED};

use kasmctl::api::KasmClient;
use kasmctl::cli::config_cmd::ConfigCommand;
use kasmctl::cli::verbs::create::CreateResource;
use kasmctl::cli::verbs::delete::DeleteResource;
use kasmctl::cli::verbs::get::GetResource;
use kasmctl::cli::{Cli, Command};
use kasmctl::config::model::{Context as KasmContext, NamedContext};
use kasmctl::config::{load_config, save_config};
use kasmctl::output::{self, OutputFormat};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Config(args) => handle_config(args.command),
        cmd => {
            let ctx =
                kasmctl::config::resolve_context(cli.server.as_deref(), cli.context.as_deref())?;
            let client = KasmClient::new(&ctx)?;

            match cmd {
                Command::Get(args) => handle_get(&client, args.resource, &cli.output).await,
                Command::Create(args) => handle_create(&client, args.resource, &cli.output).await,
                Command::Delete(args) => handle_delete(&client, args.resource).await,
                Command::Config(_) => unreachable!(),
            }
        }
    }
}

async fn handle_get(
    client: &KasmClient,
    resource: GetResource,
    format: &OutputFormat,
) -> Result<()> {
    match resource {
        GetResource::Sessions { id: None } => {
            let sessions = client
                .get_kasms()
                .await
                .context("failed to list sessions")?;
            println!("{}", output::render_list(&sessions, format)?);
        }
        GetResource::Sessions { id: Some(id) } => {
            let session = client
                .get_kasm_status(&id)
                .await
                .context("failed to get session")?;
            println!("{}", output::render_one(&session, format)?);
        }
    }
    Ok(())
}

async fn handle_create(
    client: &KasmClient,
    resource: CreateResource,
    format: &OutputFormat,
) -> Result<()> {
    match resource {
        CreateResource::Session { image, user } => {
            let resp = client
                .request_kasm(&image, user.as_deref())
                .await
                .context("failed to create session")?;

            match format {
                OutputFormat::Table => {
                    println!("Session created: {}", resp.kasm_id);
                    if let Some(url) = &resp.kasm_url {
                        println!("URL: {url}");
                    }
                    if let Some(status) = &resp.status {
                        println!("Status: {status}");
                    }
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&resp)?);
                }
                OutputFormat::Yaml => {
                    println!("{}", serde_yaml::to_string(&resp)?);
                }
            }
        }
    }
    Ok(())
}

async fn handle_delete(client: &KasmClient, resource: DeleteResource) -> Result<()> {
    match resource {
        DeleteResource::Session { id } => {
            client
                .destroy_kasm(&id)
                .await
                .context("failed to delete session")?;
            println!("Session {id} deleted.");
        }
    }
    Ok(())
}

fn handle_config(command: ConfigCommand) -> Result<()> {
    match command {
        ConfigCommand::SetContext {
            name,
            server,
            api_key,
            api_secret,
            insecure,
        } => {
            let mut config = load_config()?;

            // Update existing or add new context
            if let Some(existing) = config.contexts.iter_mut().find(|c| c.name == name) {
                existing.context = KasmContext {
                    server,
                    api_key,
                    api_secret,
                    insecure_skip_tls_verify: insecure,
                };
            } else {
                config.contexts.push(NamedContext {
                    name: name.clone(),
                    context: KasmContext {
                        server,
                        api_key,
                        api_secret,
                        insecure_skip_tls_verify: insecure,
                    },
                });
            }

            // Auto-set current context if this is the first one
            if config.current_context.is_none() {
                config.current_context = Some(name.clone());
            }

            save_config(&config)?;
            println!("Context {name:?} set.");
        }

        ConfigCommand::UseContext { name } => {
            let mut config = load_config()?;

            // Verify the context exists
            if !config.contexts.iter().any(|c| c.name == name) {
                anyhow::bail!("context {name:?} not found");
            }

            config.current_context = Some(name.clone());
            save_config(&config)?;
            println!("Switched to context {name:?}.");
        }

        ConfigCommand::GetContexts => {
            let config = load_config()?;
            let current = config.current_context.as_deref().unwrap_or("");

            let mut table = Table::new();
            table.load_preset(UTF8_FULL_CONDENSED);
            table.set_header(vec!["", "NAME", "SERVER"]);

            for ctx in &config.contexts {
                let marker = if ctx.name == current { "*" } else { "" };
                table.add_row(vec![marker, &ctx.name, &ctx.context.server]);
            }

            println!("{table}");
        }
    }
    Ok(())
}
