
mod chain;
mod wallet;
mod api;
mod node;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run node + REST API
    Node,
    /// CLI wallet commands
    Wallet(wallet::WalletCmd),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Node => node::run_node().await,
        Commands::Wallet(w) => wallet::run(w),
    }
}
