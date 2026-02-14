use clap::{Args, Subcommand};

use crate::cli::filters::SessionFilters;

#[derive(Args)]
pub struct StopArgs {
    #[command(subcommand)]
    pub resource: StopResource,
}

#[derive(Subcommand)]
pub enum StopResource {
    /// Stop a session
    #[command(alias = "kasm")]
    Session {
        /// Session ID to stop
        id: String,
    },
    /// Stop multiple sessions matching filters
    #[command(alias = "kasms")]
    Sessions {
        #[command(flatten)]
        filters: SessionFilters,
        /// Skip confirmation prompt
        #[arg(long, short)]
        yes: bool,
    },
}
