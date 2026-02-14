pub mod config_cmd;
pub mod verbs;

use clap::Parser;

use crate::output::OutputFormat;

use self::config_cmd::ConfigArgs;
use self::verbs::create::CreateArgs;
use self::verbs::delete::DeleteArgs;
use self::verbs::get::GetArgs;
use self::verbs::pause::PauseArgs;
use self::verbs::resume::ResumeArgs;
use self::verbs::stop::StopArgs;

#[derive(Parser)]
#[command(name = "kasmctl", version, about = "CLI for managing Kasm Workspaces")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output format
    #[arg(short, long, global = true, default_value = "table")]
    pub output: OutputFormat,

    /// Override the active context
    #[arg(long, global = true)]
    pub context: Option<String>,

    /// Override server URL (requires KASMCTL_API_KEY and KASMCTL_API_SECRET env vars)
    #[arg(long, global = true)]
    pub server: Option<String>,

    /// Skip TLS certificate verification (for self-signed certificates)
    #[arg(long, global = true, default_value_t = false)]
    pub insecure: bool,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Get or list resources
    Get(GetArgs),
    /// Create a resource
    Create(CreateArgs),
    /// Delete a resource
    Delete(DeleteArgs),
    /// Stop a session (frees memory and CPU, keeps disk state)
    Stop(StopArgs),
    /// Pause a session (retains memory state, stops CPU usage)
    Pause(PauseArgs),
    /// Resume a stopped or paused session
    Resume(ResumeArgs),
    /// Manage configuration contexts
    Config(ConfigArgs),
}
