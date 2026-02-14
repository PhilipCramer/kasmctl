use clap::{Args, Subcommand};

#[derive(Args)]
pub struct GetArgs {
    #[command(subcommand)]
    pub resource: GetResource,
}

#[derive(Subcommand)]
pub enum GetResource {
    /// List or get sessions
    #[command(alias = "session", alias = "kasm", alias = "kasms")]
    Sessions {
        /// Session ID to get a specific session
        id: Option<String>,
        /// Filter sessions by status (case-insensitive)
        #[arg(long)]
        status: Option<String>,
    },
    /// List available workspace images
    #[command(alias = "image")]
    Images {
        /// Image ID to get details for a specific image
        id: Option<String>,
    },
}
