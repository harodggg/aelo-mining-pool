use tokio::runtime::{self, Runtime};
pub fn runtime() -> Runtime {
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
