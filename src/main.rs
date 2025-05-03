// cargo build && cargo install --path .
mod cli;
mod commands;
mod util;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Version => {
            commands::print_version();
        }
        Commands::Install => {
            commands::run_solana_install_script();
        }
        Commands::Init(opts) => {
            commands::handle_init(opts);
        }
    }
}
