mod client;
use client::run_node_prover;
use client::run_prover;
use simple_log::{info, log::debug, LogConfigBuilder};

// todo 开放接口，将数据写到数据中，供mining pool读取。
// todo 可以通过 grpc 进行。使用3个grpc 服务进行通信。以实现代码分离和抽象。
// todo 独立二进制程序，不与pool service 一起启动，实现解除耦合。
// todo rpc 读取，和写数据库。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    debug!("run prover");
    //run_prover().await;
    run_prover().await;
    info!("Runing Stratum Service");
    Ok(())
}
