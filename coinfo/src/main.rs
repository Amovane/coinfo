use clap::Parser;
use commands::Commands;
use handlers::{handle_airdrop, handle_community, handle_ticker};
mod aggregators;
mod commands;
mod display;
mod exchanges;
mod handlers;

fn main() {
    let args = commands::Cli::parse();
    match args.command {
        Commands::Ticker(ticker_arg) => handle_ticker(ticker_arg),
        Commands::Community { currency } => handle_community(currency),
        Commands::Airdrop(airdrop_arg) => handle_airdrop(airdrop_arg),
    }
}
