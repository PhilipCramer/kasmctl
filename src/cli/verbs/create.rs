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

    /// Create a new workspace image
    Image {
        /// Docker image name (e.g. kasmweb/terminal:1.18.0)
        #[arg(long)]
        name: String,

        /// Human-readable display name
        #[arg(long)]
        friendly_name: String,

        /// Image description
        #[arg(long)]
        description: Option<String>,

        /// Number of CPU cores to allocate
        #[arg(long)]
        cores: Option<f64>,

        /// Memory in bytes to allocate
        #[arg(long)]
        memory: Option<i64>,

        /// Whether the image is enabled
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        enabled: bool,

        /// Image source type
        #[arg(long, default_value = "Container")]
        image_src: String,

        /// Docker registry URL
        #[arg(long)]
        docker_registry: Option<String>,

        /// Run configuration JSON
        #[arg(long)]
        run_config: Option<String>,

        /// Exec configuration JSON
        #[arg(long)]
        exec_config: Option<String>,

        /// Image type (e.g. Container, Server)
        #[arg(long)]
        image_type: Option<String>,
    },

    /// Create a new server
    Server {
        /// Human-readable name
        #[arg(long)]
        friendly_name: String,

        /// Server hostname or IP
        #[arg(long)]
        hostname: String,

        /// Connection type (ssh, rdp, vnc, kasmvnc)
        #[arg(long)]
        connection_type: String,

        /// Connection port
        #[arg(long)]
        connection_port: i32,

        /// Zone ID to assign the server to
        #[arg(long)]
        zone: String,

        /// Whether the server is enabled
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        enabled: bool,

        /// Connection username
        #[arg(long)]
        connection_username: Option<String>,

        /// Connection info/credentials
        #[arg(long)]
        connection_info: Option<String>,

        /// Maximum simultaneous sessions
        #[arg(long)]
        max_simultaneous_sessions: Option<i32>,

        /// Maximum simultaneous users
        #[arg(long)]
        max_simultaneous_users: Option<i32>,

        /// Pool ID
        #[arg(long)]
        pool_id: Option<String>,
    },
}
