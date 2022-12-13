use aleo_pool::rpc::run_rpc;
use aleo_pool::version::LOGO;

use aleo_utils::log::log;
use aleo_utils::print_welcome;
use simple_log::info;
use snarkvm::prelude::{EpochChallenge, FromBytes, Testnet3};
use tokio::spawn;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log().unwrap();
    print_welcome(LOGO);
    info!("Runing Mining Pool");

    // Building mpsc in many green thread, rpc thread update block, local thread send block by loop
    let (mut tx, mut rx) = mpsc::channel::<EpochChallenge<Testnet3>>(1);

    spawn(async {
        run_rpc(tx).await;
    });
    spawn(async move {
        loop {
            for r in rx.recv().await {
                info! {"Other Green Thread:{:?}",r};
            }
        }
    });

    std::future::pending::<()>().await;
    Ok(())
}
