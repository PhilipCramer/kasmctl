use clap::{Args, Subcommand};

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
    },
    /// List all sessions
    #[command(alias = "kasms")]
    Sessions {
        /// Filter sessions by status (case-insensitive)
        #[arg(long)]
        status: Option<String>,
    },
    /// Get a specific image by ID
    Image {
        /// Image ID
        id: String,
    },
    /// List all available workspace images
    Images,
}
