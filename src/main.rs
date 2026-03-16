use std::io;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use comfy_table::{Table, presets::UTF8_FULL_CONDENSED};

use kasmctl::api::KasmClient;
use kasmctl::api::agents::UpdateAgentRequest;
use kasmctl::api::images::UpdateImageRequest;
use kasmctl::api::servers::UpdateServerRequest;
use kasmctl::cli::config_cmd::ConfigCommand;
use kasmctl::cli::filters::parse_memory;
use kasmctl::cli::verbs::create::CreateResource;
use kasmctl::cli::verbs::delete::DeleteResource;
use kasmctl::cli::verbs::exec::ExecResource;
use kasmctl::cli::verbs::get::GetResource;
use kasmctl::cli::verbs::pause::PauseResource;
use kasmctl::cli::verbs::resume::ResumeResource;
use kasmctl::cli::verbs::stop::StopResource;
use kasmctl::cli::verbs::top::TopCommand;
use kasmctl::cli::verbs::update::UpdateResource;
use kasmctl::cli::{Cli, Command};
use kasmctl::config::model::{Context as KasmContext, NamedContext};
use kasmctl::config::{load_config, save_config};
use kasmctl::confirm;
use kasmctl::models::report::{HealthStatus, TopOverview};
use kasmctl::output::{self, OutputFormat};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Config(args) => handle_config(args.command),
        Command::Completion { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "kasmctl", &mut io::stdout());
            Ok(())
        }
        cmd => {
            let ctx = kasmctl::config::resolve_context(
                cli.server.as_deref(),
                cli.context.as_deref(),
                cli.insecure,
            )?;
            let client = KasmClient::new(&ctx)?;

            let context_name = if cli.server.is_some() {
                "(inline)".to_string()
            } else if let Some(name) = &cli.context {
                name.clone()
            } else if let Ok(config) = load_config() {
                config
                    .current_context
                    .unwrap_or_else(|| "(unknown)".to_string())
            } else {
                "(unknown)".to_string()
            };

            match cmd {
                Command::Get(args) => handle_get(&client, args.resource, &cli.output),
                Command::Create(args) => handle_create(&client, args.resource, &cli.output),
                Command::Delete(args) => handle_delete(&client, args.resource),
                Command::Stop(args) => handle_stop(&client, args.resource),
                Command::Pause(args) => handle_pause(&client, args.resource),
                Command::Resume(args) => handle_resume(&client, args.resource),
                Command::Update(args) => handle_update(&client, args.resource, &cli.output),
                Command::Exec(args) => handle_exec(&client, args.resource),
                Command::Health => handle_health(&client, &ctx, &context_name, &cli.output),
                Command::Top(args) => handle_top(&client, args.command, &cli.output),
                Command::Config(_) | Command::Completion { .. } => unreachable!(),
            }
        }
    }
}

fn handle_get(client: &KasmClient, resource: GetResource, format: &OutputFormat) -> Result<()> {
    match resource {
        GetResource::Session { id } => {
            let user_id = client
                .resolve_user_id(&id)
                .context("failed to resolve user for session")?;
            let session = client
                .get_kasm_status(&id, &user_id)
                .context("failed to get session")?;
            println!("{}", output::render_one(&session, format)?);
        }
        GetResource::Sessions { filters } => {
            filters.validate().map_err(|e| anyhow::anyhow!(e))?;
            let mut sessions = client.get_kasms().context("failed to list sessions")?;
            filters
                .apply(&mut sessions)
                .map_err(|e| anyhow::anyhow!(e))?;
            println!("{}", output::render_list(&sessions, format)?);
        }
        GetResource::Image { id } => {
            let image = client
                .resolve_image(&id)
                .context("failed to resolve image")?;
            println!("{}", output::render_one(&image, format)?);
        }
        GetResource::Images { filters } => {
            let mut images = client.get_images().context("failed to list images")?;
            filters.apply(&mut images);
            println!("{}", output::render_list(&images, format)?);
        }
        GetResource::Zone { id } => {
            let zone = client.resolve_zone(&id).context("failed to resolve zone")?;
            println!("{}", output::render_one(&zone, format)?);
        }
        GetResource::Zones { filters } => {
            let mut zones = client.get_zones().context("failed to list zones")?;
            filters.apply(&mut zones);
            println!("{}", output::render_list(&zones, format)?);
        }
        GetResource::Agent { id } => {
            let agent = client
                .resolve_agent(&id)
                .context("failed to resolve agent")?;
            println!("{}", output::render_one(&agent, format)?);
        }
        GetResource::Agents { filters } => {
            let mut agents = client.get_agents().context("failed to list agents")?;
            filters.apply(&mut agents);
            println!("{}", output::render_list(&agents, format)?);
        }
        GetResource::Server { id } => {
            let server = client
                .resolve_server(&id)
                .context("failed to resolve server")?;
            println!("{}", output::render_one(&server, format)?);
        }
        GetResource::Servers { filters } => {
            let mut servers = client.get_servers().context("failed to list servers")?;
            filters.apply(&mut servers);
            println!("{}", output::render_list(&servers, format)?);
        }
    }
    Ok(())
}

fn handle_create(
    client: &KasmClient,
    resource: CreateResource,
    format: &OutputFormat,
) -> Result<()> {
    match resource {
        CreateResource::Session { image, user } => {
            let resolved_image = client
                .resolve_image(&image)
                .context("failed to resolve image")?;
            let resp = client
                .request_kasm(&resolved_image.image_id, user.as_deref())
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
        CreateResource::Image {
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
        } => {
            let memory_bytes = memory
                .as_deref()
                .map(parse_memory)
                .transpose()
                .map_err(|e| anyhow::anyhow!(e))?;
            let params = kasmctl::api::images::CreateImageParams {
                name,
                friendly_name,
                description,
                cores,
                memory: memory_bytes,
                enabled,
                image_src,
                docker_registry,
                run_config,
                exec_config,
                image_type,
            };
            let image = client
                .create_image(&params)
                .context("failed to create image")?;
            println!("{}", output::render_one(&image, format)?);
        }
        CreateResource::Server {
            friendly_name,
            hostname,
            connection_type,
            connection_port,
            zone,
            enabled,
            connection_username,
            connection_info,
            max_simultaneous_sessions,
            max_simultaneous_users,
            pool_id,
        } => {
            let resolved_zone = client
                .resolve_zone(&zone)
                .context("failed to resolve zone")?;
            let params = kasmctl::api::servers::CreateServerParams {
                friendly_name,
                hostname,
                connection_type,
                connection_port,
                zone_id: resolved_zone.zone_id,
                enabled,
                connection_username,
                connection_info,
                max_simultaneous_sessions,
                max_simultaneous_users,
                pool_id,
            };
            let server = client
                .create_server(&params)
                .context("failed to create server")?;
            println!("{}", output::render_one(&server, format)?);
        }
    }
    Ok(())
}

fn handle_delete(client: &KasmClient, resource: DeleteResource) -> Result<()> {
    match resource {
        DeleteResource::Session { id, yes } => {
            let user_id = client
                .resolve_user_id(&id)
                .context("failed to resolve user for session")?;
            if !confirm::confirm(&format!("Delete session {id}?"), yes) {
                eprintln!("Aborted.");
                return Ok(());
            }
            client
                .destroy_kasm(&id, &user_id)
                .context("failed to delete session")?;
            println!("Session {id} deleted.");
        }
        DeleteResource::Image { id, yes } => {
            let image = client
                .resolve_image(&id)
                .context("failed to resolve image")?;
            let display_name = image
                .friendly_name
                .as_deref()
                .unwrap_or(&image.image_id)
                .to_string();
            if !confirm::confirm(&format!("Delete image {display_name:?}?"), yes) {
                eprintln!("Aborted.");
                return Ok(());
            }
            client
                .delete_image(&image.image_id)
                .context("failed to delete image")?;
            println!("Image {display_name:?} deleted.");
        }
        DeleteResource::Server { id, yes } => {
            let server = client
                .resolve_server(&id)
                .context("failed to resolve server")?;
            let display_name = server
                .friendly_name
                .as_deref()
                .unwrap_or(&server.server_id)
                .to_string();
            if !confirm::confirm(&format!("Delete server {display_name:?}?"), yes) {
                eprintln!("Aborted.");
                return Ok(());
            }
            client
                .delete_server(&server.server_id)
                .context("failed to delete server")?;
            println!("Server {display_name:?} deleted.");
        }
    }
    Ok(())
}

fn handle_update(
    client: &KasmClient,
    resource: UpdateResource,
    format: &OutputFormat,
) -> Result<()> {
    match resource {
        UpdateResource::Image {
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
        } => {
            let resolved = client
                .resolve_image(&id)
                .context("failed to resolve image")?;
            let memory_bytes = memory
                .as_deref()
                .map(parse_memory)
                .transpose()
                .map_err(|e| anyhow::anyhow!(e))?;
            let req = UpdateImageRequest {
                image_id: resolved.image_id,
                name,
                friendly_name,
                description,
                cores,
                memory: memory_bytes,
                enabled,
                image_src,
                docker_registry,
                run_config,
                exec_config,
                hidden,
            };
            let image = client
                .update_image(&req)
                .context("failed to update image")?;
            println!("{}", output::render_one(&image, format)?);
        }
        UpdateResource::Agent {
            id,
            enabled,
            cores_override,
            memory_override,
            gpus_override,
            auto_prune_images,
        } => {
            let resolved = client
                .resolve_agent(&id)
                .context("failed to resolve agent")?;
            let memory_override_bytes = memory_override
                .as_deref()
                .map(parse_memory)
                .transpose()
                .map_err(|e| anyhow::anyhow!(e))?;
            let req = UpdateAgentRequest {
                agent_id: resolved.agent_id,
                enabled,
                cores_override,
                memory_override: memory_override_bytes,
                gpus_override,
                auto_prune_images,
            };
            let agent = client
                .update_agent(&req)
                .context("failed to update agent")?;
            println!("{}", output::render_one(&agent, format)?);
        }
        UpdateResource::Server {
            id,
            friendly_name,
            hostname,
            enabled,
            connection_type,
            connection_port,
            connection_username,
            connection_info,
            max_simultaneous_sessions,
            max_simultaneous_users,
            zone_id,
            pool_id,
        } => {
            let resolved = client
                .resolve_server(&id)
                .context("failed to resolve server")?;
            let resolved_zone_id = zone_id
                .as_deref()
                .map(|z| client.resolve_zone(z).map(|zone| zone.zone_id))
                .transpose()
                .context("failed to resolve zone")?;
            let req = UpdateServerRequest {
                server_id: resolved.server_id,
                friendly_name,
                hostname,
                enabled,
                connection_type,
                connection_port,
                connection_username,
                connection_info,
                max_simultaneous_sessions,
                max_simultaneous_users,
                zone_id: resolved_zone_id,
                pool_id,
            };
            let server = client
                .update_server(&req)
                .context("failed to update server")?;
            println!("{}", output::render_one(&server, format)?);
        }
    }
    Ok(())
}

fn handle_stop(client: &KasmClient, resource: StopResource) -> Result<()> {
    match resource {
        StopResource::Session { id } => {
            let user_id = client
                .resolve_user_id(&id)
                .context("failed to resolve user for session")?;
            client
                .stop_kasm(&id, &user_id)
                .context("failed to stop session")?;
            println!("Session {id} stopped.");
        }
        StopResource::Sessions { filters, yes } => {
            filters.validate().map_err(|e| anyhow::anyhow!(e))?;
            let mut sessions = client.get_kasms().context("failed to list sessions")?;
            filters
                .apply(&mut sessions)
                .map_err(|e| anyhow::anyhow!(e))?;

            if sessions.is_empty() {
                eprintln!("No sessions match the given filters.");
                return Ok(());
            }

            let msg = if filters.is_empty() {
                format!("Stop ALL {} sessions?", sessions.len())
            } else {
                format!("Stop {} matching sessions?", sessions.len())
            };
            if !confirm::confirm(&msg, yes) {
                eprintln!("Aborted.");
                return Ok(());
            }

            let total = sessions.len();
            let mut failed = 0usize;
            let mut skipped = 0usize;
            for s in &sessions {
                let op_status = s.operational_status.as_deref().unwrap_or("");
                if op_status.eq_ignore_ascii_case("stopped") {
                    eprintln!("  {} skipped ({op_status})", s.kasm_id);
                    skipped += 1;
                    continue;
                }
                let user_id = match s.user_id.as_deref() {
                    Some(uid) => uid,
                    None => {
                        eprintln!("  {} skipped (no user_id)", s.kasm_id);
                        skipped += 1;
                        continue;
                    }
                };
                match client.stop_kasm(&s.kasm_id, user_id) {
                    Ok(()) => eprintln!("  {} ok", s.kasm_id),
                    Err(e) => {
                        eprintln!("  {} FAILED: {e}", s.kasm_id);
                        failed += 1;
                    }
                }
            }

            let attempted = total - skipped;
            eprintln!(
                "Stopped {}/{} sessions.{}",
                attempted - failed,
                attempted,
                if skipped > 0 {
                    format!(" ({skipped} skipped)")
                } else {
                    String::new()
                }
            );
            if failed > 0 {
                anyhow::bail!("{failed} session(s) failed to stop");
            }
        }
    }
    Ok(())
}

fn handle_pause(client: &KasmClient, resource: PauseResource) -> Result<()> {
    match resource {
        PauseResource::Session { id } => {
            let user_id = client
                .resolve_user_id(&id)
                .context("failed to resolve user for session")?;
            client
                .pause_kasm(&id, &user_id)
                .context("failed to pause session")?;
            println!("Session {id} paused.");
        }
        PauseResource::Sessions { filters, yes } => {
            filters.validate().map_err(|e| anyhow::anyhow!(e))?;
            let mut sessions = client.get_kasms().context("failed to list sessions")?;
            filters
                .apply(&mut sessions)
                .map_err(|e| anyhow::anyhow!(e))?;

            if sessions.is_empty() {
                eprintln!("No sessions match the given filters.");
                return Ok(());
            }

            let msg = if filters.is_empty() {
                format!("Pause ALL {} sessions?", sessions.len())
            } else {
                format!("Pause {} matching sessions?", sessions.len())
            };
            if !confirm::confirm(&msg, yes) {
                eprintln!("Aborted.");
                return Ok(());
            }

            let total = sessions.len();
            let mut failed = 0usize;
            let mut skipped = 0usize;
            for s in &sessions {
                let op_status = s.operational_status.as_deref().unwrap_or("");
                if op_status.eq_ignore_ascii_case("stopped")
                    || op_status.eq_ignore_ascii_case("paused")
                {
                    eprintln!("  {} skipped ({op_status})", s.kasm_id);
                    skipped += 1;
                    continue;
                }
                let user_id = match s.user_id.as_deref() {
                    Some(uid) => uid,
                    None => {
                        eprintln!("  {} skipped (no user_id)", s.kasm_id);
                        skipped += 1;
                        continue;
                    }
                };
                match client.pause_kasm(&s.kasm_id, user_id) {
                    Ok(()) => eprintln!("  {} ok", s.kasm_id),
                    Err(e) => {
                        eprintln!("  {} FAILED: {e}", s.kasm_id);
                        failed += 1;
                    }
                }
            }

            let attempted = total - skipped;
            eprintln!(
                "Paused {}/{} sessions.{}",
                attempted - failed,
                attempted,
                if skipped > 0 {
                    format!(" ({skipped} skipped)")
                } else {
                    String::new()
                }
            );
            if failed > 0 {
                anyhow::bail!("{failed} session(s) failed to pause");
            }
        }
    }
    Ok(())
}

fn handle_resume(client: &KasmClient, resource: ResumeResource) -> Result<()> {
    match resource {
        ResumeResource::Session { id } => {
            let user_id = client
                .resolve_user_id(&id)
                .context("failed to resolve user for session")?;
            client
                .resume_kasm(&id, &user_id)
                .context("failed to resume session")?;
            println!("Session {id} resumed.");
        }
        ResumeResource::Sessions { filters, yes } => {
            filters.validate().map_err(|e| anyhow::anyhow!(e))?;
            let mut sessions = client.get_kasms().context("failed to list sessions")?;
            filters
                .apply(&mut sessions)
                .map_err(|e| anyhow::anyhow!(e))?;

            if sessions.is_empty() {
                eprintln!("No sessions match the given filters.");
                return Ok(());
            }

            let msg = if filters.is_empty() {
                format!("Resume ALL {} sessions?", sessions.len())
            } else {
                format!("Resume {} matching sessions?", sessions.len())
            };
            if !confirm::confirm(&msg, yes) {
                eprintln!("Aborted.");
                return Ok(());
            }

            let total = sessions.len();
            let mut failed = 0usize;
            let mut skipped = 0usize;
            for s in &sessions {
                let op_status = s.operational_status.as_deref().unwrap_or("");
                if op_status.eq_ignore_ascii_case("running") {
                    eprintln!("  {} skipped ({op_status})", s.kasm_id);
                    skipped += 1;
                    continue;
                }
                let user_id = match s.user_id.as_deref() {
                    Some(uid) => uid,
                    None => {
                        eprintln!("  {} skipped (no user_id)", s.kasm_id);
                        skipped += 1;
                        continue;
                    }
                };
                match client.resume_kasm(&s.kasm_id, user_id) {
                    Ok(()) => eprintln!("  {} ok", s.kasm_id),
                    Err(e) => {
                        eprintln!("  {} FAILED: {e}", s.kasm_id);
                        failed += 1;
                    }
                }
            }

            let attempted = total - skipped;
            eprintln!(
                "Resumed {}/{} sessions.{}",
                attempted - failed,
                attempted,
                if skipped > 0 {
                    format!(" ({skipped} skipped)")
                } else {
                    String::new()
                }
            );
            if failed > 0 {
                anyhow::bail!("{failed} session(s) failed to resume");
            }
        }
    }
    Ok(())
}

fn handle_exec(client: &KasmClient, resource: ExecResource) -> Result<()> {
    match resource {
        ExecResource::Session {
            id,
            workdir,
            privileged,
            exec_user,
            cmd,
        } => {
            let user_id = client
                .resolve_user_id(&id)
                .context("failed to resolve user for session")?;
            let cmd_str = cmd.join(" ");
            client
                .exec_command_kasm(
                    &id,
                    &user_id,
                    &cmd_str,
                    workdir.as_deref(),
                    privileged,
                    exec_user.as_deref(),
                )
                .context("failed to execute command")?;
            println!("Command submitted to session {id}.");
        }
        ExecResource::Sessions {
            filters,
            yes,
            workdir,
            privileged,
            exec_user,
            cmd,
        } => {
            filters.validate().map_err(|e| anyhow::anyhow!(e))?;
            let mut sessions = client.get_kasms().context("failed to list sessions")?;
            filters
                .apply(&mut sessions)
                .map_err(|e| anyhow::anyhow!(e))?;

            if sessions.is_empty() {
                eprintln!("No sessions match the given filters.");
                return Ok(());
            }

            let msg = if filters.is_empty() {
                format!("Execute command on ALL {} sessions?", sessions.len())
            } else {
                format!("Execute command on {} matching sessions?", sessions.len())
            };
            if !confirm::confirm(&msg, yes) {
                eprintln!("Aborted.");
                return Ok(());
            }

            let cmd_str = cmd.join(" ");
            let total = sessions.len();
            let mut failed = 0usize;
            let mut skipped = 0usize;
            for s in &sessions {
                let user_id = match &s.user_id {
                    Some(uid) => uid,
                    None => {
                        eprintln!("  {} skipped (no user_id)", s.kasm_id);
                        skipped += 1;
                        continue;
                    }
                };
                match client.exec_command_kasm(
                    &s.kasm_id,
                    user_id,
                    &cmd_str,
                    workdir.as_deref(),
                    privileged,
                    exec_user.as_deref(),
                ) {
                    Ok(()) => eprintln!("  {} ok", s.kasm_id),
                    Err(e) => {
                        eprintln!("  {} FAILED: {e}", s.kasm_id);
                        failed += 1;
                    }
                }
            }

            let attempted = total - skipped;
            eprintln!(
                "Executed on {}/{} sessions.{}",
                attempted - failed,
                attempted,
                if skipped > 0 {
                    format!(" ({skipped} skipped)")
                } else {
                    String::new()
                }
            );
            if failed > 0 {
                anyhow::bail!("{failed} session(s) failed to execute");
            }
        }
    }
    Ok(())
}

fn handle_health(
    client: &KasmClient,
    ctx: &KasmContext,
    context_name: &str,
    format: &OutputFormat,
) -> Result<()> {
    let start = Instant::now();
    match client.get_report("current_kasms", Some(1), None) {
        Ok(report) => {
            let elapsed = start.elapsed().as_millis() as u64;
            let health = HealthStatus {
                server: ctx.server.clone(),
                context: context_name.to_string(),
                status: "OK".to_string(),
                error: None,
                latency_ms: Some(elapsed),
                sessions: report.as_u64(),
            };
            print_health(&health, format)?;
        }
        Err(e) => {
            let health = HealthStatus {
                server: ctx.server.clone(),
                context: context_name.to_string(),
                status: "ERROR".to_string(),
                error: Some(e.to_string()),
                latency_ms: None,
                sessions: None,
            };
            print_health(&health, format)?;
            std::process::exit(1);
        }
    }
    Ok(())
}

fn print_health(health: &HealthStatus, format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Table => {
            println!("Server:    {}", health.server);
            println!("Context:   {}", health.context);
            println!("Status:    {}", health.status);
            if let Some(ms) = health.latency_ms {
                println!("Latency:   {ms}ms");
            }
            if let Some(n) = health.sessions {
                println!("Sessions:  {n} running");
            }
            if let Some(err) = &health.error {
                println!("Error:     {err}");
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(health)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(health)?);
        }
    }
    Ok(())
}

fn handle_top(
    client: &KasmClient,
    command: Option<TopCommand>,
    format: &OutputFormat,
) -> Result<()> {
    match command {
        Some(TopCommand::Agents) => {
            let agents = client
                .get_agent_report()
                .context("failed to get agent report")?;
            println!("{}", output::render_list(&agents, format)?);
        }
        None => {
            let kasms = client.get_kasms().context("failed to list sessions")?;
            let sessions = kasms.len() as u64;
            let users = {
                let mut ids: Vec<&str> =
                    kasms.iter().filter_map(|k| k.user_id.as_deref()).collect();
                ids.sort_unstable();
                ids.dedup();
                ids.len() as u64
            };
            let errors = client
                .get_report("get_errors", Some(86400), None)
                .context("failed to get error count")?
                .as_u64()
                .unwrap_or(0);
            let agents = client
                .get_agent_report()
                .context("failed to get agent report")?;

            match format {
                OutputFormat::Table => {
                    println!(
                        "Sessions: {sessions} running    Users: {users} connected    Errors: {errors} (24h)"
                    );
                    println!();
                    println!("{}", output::render_list(&agents, format)?);
                }
                OutputFormat::Json => {
                    let overview = TopOverview {
                        sessions,
                        users,
                        errors,
                        agents,
                    };
                    println!("{}", serde_json::to_string_pretty(&overview)?);
                }
                OutputFormat::Yaml => {
                    let overview = TopOverview {
                        sessions,
                        users,
                        errors,
                        agents,
                    };
                    println!("{}", serde_yaml::to_string(&overview)?);
                }
            }
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
                    timeout_seconds: None,
                };
            } else {
                config.contexts.push(NamedContext {
                    name: name.clone(),
                    context: KasmContext {
                        server,
                        api_key,
                        api_secret,
                        insecure_skip_tls_verify: insecure,
                        timeout_seconds: None,
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
