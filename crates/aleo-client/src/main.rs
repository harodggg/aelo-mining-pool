mod client;
use client::run_prover;
use client::ClientRpc;
use simple_log::{info, log::debug, LogConfigBuilder};
use tokio::runtime::{self, Runtime};

// todo 开放接口，将数据写到数据中，供mining pool读取。
// todo 可以通过 grpc 进行。使用3个grpc 服务进行通信。以实现代码分离和抽象。
// todo 独立二进制程序，不与pool service 一起启动，实现解除耦合。
// todo rpc 读取，和写数据库。
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = LogConfigBuilder::builder()
        .path("./log/builder_log.log")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("info")
        .output_file()
        .output_console()
        .build();
    simple_log::new(config)?;
    debug!("run prover");
    // ClientRpc::run().await?;
    runtime().block_on(async move {
        run_prover().await.unwrap();
        std::future::pending::<()>().await;
    });
    Ok(())
}

fn runtime() -> Runtime {
    // TODO (howardwu): Fix this.
    // let (num_tokio_worker_threads, max_tokio_blocking_threads, num_rayon_cores_global) = if !Self::node_type().is_beacon() {
    //     ((num_cpus::get() / 8 * 2).max(1), num_cpus::get(), (num_cpus::get() / 8 * 5).max(1))
    // } else {
    //     (num_cpus::get(), 512, num_cpus::get()) // 512 is tokio's current default
    // };
    let (num_tokio_worker_threads, max_tokio_blocking_threads, num_rayon_cores_global) =
            // { ((num_cpus::get() / 2).max(1), num_cpus::get(), (num_cpus::get() / 4 * 3).max(1)) };
            { (num_cpus::get().min(8), 512, num_cpus::get().saturating_sub(8).max(1)) };

    // Initialize the parallelization parameters.
    rayon::ThreadPoolBuilder::new()
        .stack_size(8 * 1024 * 1024)
        .num_threads(num_rayon_cores_global)
        .build_global()
        .unwrap();

    // Initialize the runtime configuration.
    runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .worker_threads(num_tokio_worker_threads)
        .max_blocking_threads(max_tokio_blocking_threads)
        .build()
        .expect("Failed to initialize a runtime for the router")
}
