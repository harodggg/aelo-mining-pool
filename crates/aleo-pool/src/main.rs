use aleo_pool::rpc::run_rpc;
use aleo_pool::version::LOGO;

use aleo_utils::log::log;
use aleo_utils::print_welcome;
use simple_log::info;
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log().unwrap();
    print_welcome(LOGO);
    info!("Runing Mining Pool");
    spawn(async {
        run_rpc().await;
    });

    std::future::pending::<()>().await;
    Ok(())
}
