use clap::{Args, Subcommand};

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
}
