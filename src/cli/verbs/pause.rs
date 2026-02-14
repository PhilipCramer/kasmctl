use clap::{Args, Subcommand};

use crate::cli::filters::SessionFilters;

#[derive(Args)]
pub struct PauseArgs {
    #[command(subcommand)]
    pub resource: PauseResource,
}

#[derive(Subcommand)]
pub enum PauseResource {
    /// Pause a session
    #[command(alias = "kasm")]
    Session {
        /// Session ID to pause
        id: String,
    },
    /// Pause multiple sessions matching filters
    #[command(alias = "kasms")]
    Sessions {
        #[command(flatten)]
        filters: SessionFilters,
        /// Skip confirmation prompt
        #[arg(long, short)]
        yes: bool,
    },
}
