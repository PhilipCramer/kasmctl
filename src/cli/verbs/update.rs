use clap::{Args, Subcommand};

#[derive(Args)]
pub struct UpdateArgs {
    #[command(subcommand)]
    pub resource: UpdateResource,
}

#[derive(Subcommand)]
pub enum UpdateResource {
    /// Update an existing workspace image
    Image {
        /// Image ID to update
        id: String,

        /// Docker image name (e.g. kasmweb/ubuntu-noble-desktop:1.18.0)
        #[arg(long)]
        name: Option<String>,

        /// Human-readable display name
        #[arg(long)]
        friendly_name: Option<String>,

        /// Image description
        #[arg(long)]
        description: Option<String>,

        /// Number of CPU cores
        #[arg(long)]
        cores: Option<f64>,

        /// Memory in bytes
        #[arg(long)]
        memory: Option<i64>,

        /// Enable or disable the image
        #[arg(long)]
        enabled: Option<bool>,

        /// Image thumbnail source path
        #[arg(long)]
        image_src: Option<String>,

        /// Docker registry URL
        #[arg(long)]
        docker_registry: Option<String>,

        /// Docker run config override (JSON)
        #[arg(long)]
        run_config: Option<String>,

        /// Docker exec config override (JSON)
        #[arg(long)]
        exec_config: Option<String>,

        /// Hide the image from users
        #[arg(long)]
        hidden: Option<bool>,
    },
    /// Update a docker agent
    #[command(alias = "docker-agent")]
    Agent {
        /// Agent ID to update
        id: String,

        /// Enable or disable the agent
        #[arg(long)]
        enabled: Option<bool>,

        /// Override CPU cores allocation
        #[arg(long)]
        cores_override: Option<f64>,

        /// Override memory allocation in bytes
        #[arg(long)]
        memory_override: Option<i64>,

        /// Override GPU allocation
        #[arg(long)]
        gpus_override: Option<f64>,

        /// Auto-prune images policy
        #[arg(long)]
        auto_prune_images: Option<String>,
    },

    /// Update an existing server
    Server {
        /// Server ID to update
        id: String,

        /// Human-readable name
        #[arg(long)]
        friendly_name: Option<String>,

        /// Server hostname or IP
        #[arg(long)]
        hostname: Option<String>,

        /// Enable or disable the server
        #[arg(long)]
        enabled: Option<bool>,

        /// Connection type
        #[arg(long)]
        connection_type: Option<String>,

        /// Connection port
        #[arg(long)]
        connection_port: Option<i32>,

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

        /// Zone ID
        #[arg(long)]
        zone_id: Option<String>,

        /// Pool ID
        #[arg(long)]
        pool_id: Option<String>,
    },
}
