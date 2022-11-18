mod db;
mod prover;
mod rpc;

use clap::Parser;
use prover::run_prover;
use stratum::run_stratum_service;

use simple_log::LogConfigBuilder;
use simple_log::{debug, info, warn};

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
    let config = LogConfigBuilder::builder()
        .path("./log/builder_log.log")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .output_console()
        .build();
    simple_log::new(config)?;
    debug!("test builder debug");

    run_prover().await;
    warn!("Runing Prover");
    run_stratum_service().await?;
    info!("Runing Stratum Service");
    if args.start {
        println!("start");
    }
    Ok(())
}
