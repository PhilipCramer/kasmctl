use clap::{Args, Subcommand};

use crate::cli::filters::SessionFilters;

#[derive(Args)]
pub struct ExecArgs {
    #[command(subcommand)]
    pub resource: ExecResource,
}

#[derive(Subcommand)]
pub enum ExecResource {
    /// Execute a command inside a session
    #[command(alias = "kasm")]
    Session {
        /// Session ID
        id: String,
        /// Working directory for the command
        #[arg(long)]
        workdir: Option<String>,
        /// Run as privileged
        #[arg(long)]
        privileged: bool,
        /// User to run the command as inside the container
        #[arg(long)]
        exec_user: Option<String>,
        /// Command to execute (after --)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        cmd: Vec<String>,
    },
    /// Execute a command across multiple sessions
    #[command(alias = "kasms")]
    Sessions {
        #[command(flatten)]
        filters: SessionFilters,
        /// Skip confirmation prompt
        #[arg(long, short)]
        yes: bool,
        /// Working directory for the command
        #[arg(long)]
        workdir: Option<String>,
        /// Run as privileged
        #[arg(long)]
        privileged: bool,
        /// User to run the command as inside the container
        #[arg(long)]
        exec_user: Option<String>,
        /// Command to execute (after --)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        cmd: Vec<String>,
    },
}
