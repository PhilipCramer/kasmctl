use clap::Subcommand;

#[derive(clap::Args)]
pub struct TopArgs {
    #[command(subcommand)]
    pub command: Option<TopCommand>,
}

#[derive(Subcommand)]
pub enum TopCommand {
    /// Show agent resource utilization
    Agents,
}
