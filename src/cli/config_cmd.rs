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

        /// API key
        #[arg(long)]
        api_key: String,

        /// API key secret
        #[arg(long)]
        api_secret: String,

        /// Skip TLS certificate verification
        #[arg(long, default_value_t = false)]
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
