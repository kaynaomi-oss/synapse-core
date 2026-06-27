mod client;
mod formatter;
mod commands;

use anyhow::Result;
use clap::Parser;
use commands::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let base_url = cli.base_url.trim_end_matches('/').to_string();
    let api_key = cli.api_key.clone();

    match cli.command {
        Commands::Health(cmd) => commands::health::run(cmd, &base_url, &api_key).await,
        Commands::Stats(cmd) => commands::stats::run(cmd, &base_url, &api_key).await,
    }
}
