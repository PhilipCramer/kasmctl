use clap::{Args, Subcommand};

use crate::cli::filters::SessionFilters;

#[derive(Args)]
pub struct GetArgs {
    #[command(subcommand)]
    pub resource: GetResource,
}

#[derive(Subcommand)]
pub enum GetResource {
    /// Get a specific session by ID
    #[command(alias = "kasm")]
    Session {
        /// Session ID
        id: String,
        /// User ID that owns the session
        #[arg(long)]
        user: String,
    },
    /// List all sessions
    #[command(alias = "kasms")]
    Sessions {
        #[command(flatten)]
        filters: SessionFilters,
    },
    /// Get a specific image by ID
    Image {
        /// Image ID
        id: String,
    },
    /// List all available workspace images
    Images,
}
