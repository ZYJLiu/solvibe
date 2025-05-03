use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show version information
    Version,
    /// Install all local dependencies for Solana development
    Install,
    /// Select and clone a Solana starter template
    Init(InitOptions),
}

#[derive(Args, Debug)]
pub struct InitOptions {
    /// Clone only the client code
    #[arg(long, conflicts_with = "full")]
    pub client: bool,
    /// Clone the fullstack program with frontend
    #[arg(long, conflicts_with = "client")]
    pub full: bool,
}
