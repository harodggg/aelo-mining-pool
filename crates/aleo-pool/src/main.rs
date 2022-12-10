
use aleo_pool::rpc::run_rpc;
use aleo_pool::version::LOGO;

use aleo_utils::log::log;
use aleo_utils::print_welcome;
use clap::Parser;
use simple_log::info;

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
    print_welcome(LOGO);
    info!("Runing Stratum Service");
    run_rpc().await;
    if args.start {
        println!("start");
    }
    Ok(())
}
