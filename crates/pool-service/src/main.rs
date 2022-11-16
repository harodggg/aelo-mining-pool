mod db;
mod rpc;

use clap::Parser;
use db::fetch_an_integer;
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
    fetch_an_integer().unwrap();
    run_stratum_service().await?;
    if args.start {
        println!("start");
    }
    Ok(())
}
