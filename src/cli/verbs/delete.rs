use clap::{Args, Subcommand};

#[derive(Args)]
pub struct DeleteArgs {
    #[command(subcommand)]
    pub resource: DeleteResource,
}

#[derive(Subcommand)]
pub enum DeleteResource {
    /// Delete a session
    #[command(alias = "kasm")]
    Session {
        /// Session ID to delete
        id: String,
    },
    /// Delete an image
    Image {
        /// Image ID to delete
        id: String,
    },
    /// Delete a server
    Server {
        /// Server ID to delete
        id: String,
    },
}
