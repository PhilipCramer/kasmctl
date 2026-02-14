pub mod config_cmd;
pub mod verbs;

use clap::Parser;

use crate::output::OutputFormat;

use self::config_cmd::ConfigArgs;
use self::verbs::create::CreateArgs;
use self::verbs::delete::DeleteArgs;
use self::verbs::get::GetArgs;

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
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Get or list resources
    Get(GetArgs),
    /// Create a resource
    Create(CreateArgs),
    /// Delete a resource
    Delete(DeleteArgs),
    /// Manage configuration contexts
    Config(ConfigArgs),
}
