use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Set a context with server URL and credentials
    SetContext {
        /// Context name
        name: String,

        /// Kasm server URL
        #[arg(long)]
        server: String,

        /// API key (reads from KASMCTL_API_KEY env var if not provided)
        #[arg(long, env = "KASMCTL_API_KEY")]
        api_key: String,

        /// API key secret (reads from KASMCTL_API_SECRET env var if not provided)
        #[arg(long, env = "KASMCTL_API_SECRET")]
        api_secret: String,

        /// Skip TLS certificate verification (for self-signed certificates)
        #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
        insecure: bool,
    },

    /// Switch the active context
    UseContext {
        /// Context name to activate
        name: String,
    },

    /// List all configured contexts
    GetContexts,
}
