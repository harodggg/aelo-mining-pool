mod client;
mod version;
use aleo_utils::log::log;
use aleo_utils::print_welcome;
//use client::run_prover;
//use client::ClientRpc;
use simple_log::{info, log::debug, LogConfigBuilder};
use tokio::runtime::{self, Runtime};
use version::LOGO;

// todo 开放接口，将数据写到数据中，供mining pool读取。
// todo 可以通过 grpc 进行。使用3个grpc 服务进行通信。以实现代码分离和抽象。
// todo 独立二进制程序，不与pool service 一起启动，实现解除耦合。
// todo rpc 读取，和写数据库。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log().unwrap();
    print_welcome(LOGO);
    info!("Start Run Client");
    //ClientRpc::run().await?;
    //run_prover(Some(5)).await?;

    std::future::pending::<()>().await;
    Ok(())
}
