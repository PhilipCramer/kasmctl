use clap::{Args, Subcommand};

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
}
