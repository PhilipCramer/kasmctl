use clap::{Args, Subcommand};

use crate::cli::filters::{AgentFilters, ImageFilters, ServerFilters, SessionFilters, ZoneFilters};

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
        #[command(flatten)]
        filters: SessionFilters,
    },
    /// Get a specific image by ID, ID prefix, or friendly name
    Image {
        /// Image to look up (exact ID, ID prefix, or case-insensitive friendly name)
        id: String,
    },
    /// List all available workspace images
    Images {
        #[command(flatten)]
        filters: ImageFilters,
    },
    /// Get a specific zone by ID, ID prefix, or name
    Zone {
        /// Zone to look up (exact ID, ID prefix, or case-insensitive zone name)
        id: String,
    },
    /// List all zones
    Zones {
        #[command(flatten)]
        filters: ZoneFilters,
    },
    /// Get a specific docker agent by ID, ID prefix, or hostname
    #[command(alias = "docker-agent")]
    Agent {
        /// Agent to look up (exact ID, ID prefix, or case-insensitive hostname)
        id: String,
    },
    /// List all docker agents
    #[command(alias = "docker-agents")]
    Agents {
        #[command(flatten)]
        filters: AgentFilters,
    },
    /// Get a specific server by ID, ID prefix, friendly name, or hostname
    Server {
        /// Server to look up (exact ID, ID prefix, case-insensitive friendly name, or hostname)
        id: String,
    },
    /// List all servers
    Servers {
        #[command(flatten)]
        filters: ServerFilters,
    },
}
