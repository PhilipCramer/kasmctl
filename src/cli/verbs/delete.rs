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
    /// Delete an image by ID, ID prefix, or friendly name
    Image {
        /// Image to delete (exact ID, ID prefix, or case-insensitive friendly name)
        id: String,
    },
    /// Delete a server
    Server {
        /// Server ID to delete
        id: String,
    },
}
