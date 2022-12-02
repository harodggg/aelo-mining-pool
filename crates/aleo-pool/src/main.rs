use aleo_pool::rpc::run_aleo_block;
use aleo_utils::log::log;
use clap::Parser;
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
    log().unwrap();
    info!("Runing Stratum Service");
    run_aleo_block().await;
    if args.start {
        println!("start");
    }
    Ok(())
}
