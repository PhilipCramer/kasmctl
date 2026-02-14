use clap::{Args, Subcommand};

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
}
