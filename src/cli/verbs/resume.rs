use clap::{Args, Subcommand};

use crate::cli::filters::SessionFilters;

#[derive(Args)]
pub struct ResumeArgs {
    #[command(subcommand)]
    pub resource: ResumeResource,
}

#[derive(Subcommand)]
pub enum ResumeResource {
    /// Resume a session
    #[command(alias = "kasm")]
    Session {
        /// Session ID to resume
        id: String,
    },
    /// Resume multiple sessions matching filters
    #[command(alias = "kasms")]
    Sessions {
        #[command(flatten)]
        filters: SessionFilters,
        /// Skip confirmation prompt
        #[arg(long, short)]
        yes: bool,
    },
}
