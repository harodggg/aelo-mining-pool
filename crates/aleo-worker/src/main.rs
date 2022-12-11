use aleo_utils::{log::log, print_welcome};
use aleo_worker::rpc::rpc_client_run;
use aleo_worker::version::LOGO;
use clap::Parser;
use simple_log::{debug, info, warn};
use stratum_worker::run_stratum_service;
use tokio;
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
    log()?;
    print_welcome(LOGO);
    info!("Starting Mining Working");
    //run_stratum_service().await?;
    let handler = tokio::spawn(async { run_stratum_service().await });
    info!("Starting Found Solution");
    rpc_client_run().await?;
    std::future::pending::<()>().await;
    if args.start {
        println!("start");
    }
    Ok(())
}
