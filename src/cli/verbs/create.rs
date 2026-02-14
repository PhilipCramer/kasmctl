use clap::{Args, Subcommand};

#[derive(Args)]
pub struct CreateArgs {
    #[command(subcommand)]
    pub resource: CreateResource,
}

#[derive(Subcommand)]
pub enum CreateResource {
    /// Create a new session
    #[command(alias = "kasm")]
    Session {
        /// Workspace image ID to launch
        #[arg(long)]
        image: String,

        /// User ID (uses API key owner if omitted)
        #[arg(long)]
        user: Option<String>,
    },
}
