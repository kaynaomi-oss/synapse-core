use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "synapse")]
#[command(about = "Synapse CLI for interacting with the Synapse API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage transactions
    Transactions {
        #[command(subcommand)]
        command: TransactionCommand,
    },
}

#[derive(Subcommand)]
enum TransactionCommand {
    /// Get a transaction by ID
    Get {
        /// Transaction ID
        id: String,
        /// Output format (table or json)
        #[arg(long, default_value = "table")]
        format: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Transactions { command } => match command {
            TransactionCommand::Get { id, format } => {
                println!("Get transaction: {} (format: {})", id, format);
            }
        },
    }
}
