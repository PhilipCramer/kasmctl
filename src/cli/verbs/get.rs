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
    },
}
