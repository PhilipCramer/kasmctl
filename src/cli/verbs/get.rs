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
        /// User ID that owns the session
        #[arg(long)]
        user: String,
    },
    /// List all sessions
    #[command(alias = "kasms")]
    Sessions {
        #[command(flatten)]
        filters: SessionFilters,
    },
    /// Get a specific image by ID
    Image {
        /// Image ID
        id: String,
    },
    /// List all available workspace images
    Images {
        #[command(flatten)]
        filters: ImageFilters,
    },
    /// Get a specific zone by ID
    Zone {
        /// Zone ID
        id: String,
    },
    /// List all zones
    Zones {
        #[command(flatten)]
        filters: ZoneFilters,
    },
    /// Get a specific docker agent by ID
    #[command(alias = "docker-agent")]
    Agent {
        /// Agent ID
        id: String,
    },
    /// List all docker agents
    #[command(alias = "docker-agents")]
    Agents {
        #[command(flatten)]
        filters: AgentFilters,
    },
    /// Get a specific server by ID
    Server {
        /// Server ID
        id: String,
    },
    /// List all servers
    Servers {
        #[command(flatten)]
        filters: ServerFilters,
    },
}
