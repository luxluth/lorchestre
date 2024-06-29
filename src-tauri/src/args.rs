use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct LorArgs {
    #[clap(subcommand)]
    pub entity: Option<LorSubcommand>,
}
#[derive(Debug, Subcommand)]
pub enum LorSubcommand {
    /// Start the Lorchestre daemon
    Daemon,
}
