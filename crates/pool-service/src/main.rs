mod db;
mod prover;
mod rpc;

use clap::Parser;
use prover::run_prover;
use stratum::run_stratum_service;

#[derive(clap::ValueEnum, Clone)]
enum State {
    Run,
    Stop,
    Pause,
}

/// Aelo Mining pool service program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Starting a aleo mining pool service
    #[arg(long)]
    start: bool,

    /// Stoping a aleo mining pool service
    #[arg(long)]
    stop: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run_prover().await;
    run_stratum_service().await?;
    if args.start {
        println!("start");
    }
    Ok(())
}
