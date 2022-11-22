use core::str::FromStr;
use snarkos_node::Node;
use snarkvm::prelude::{PrivateKey, Testnet3};
use std::net::SocketAddr;
use tokio::runtime::{self, Runtime};

type CurrentNetwork = Testnet3;

const PRIVATE_KEY: &str = "APrivateKey1zkp7rs3Ls2qGjUKiTeEgP5DrLpowzXKgZ59uk4aGqWaNvev";
const VIEW_KEY: &str = "AViewKey1eq6impGbR8JoGvmU45gSWR1KBMH1uVoLmLp9hK65LqG4";
const ADDRESS: &str = "aleo1n3dx8azjks2vlnluyelxvzys5cd3tn3jqz4m82ln9g6uy4dsd5fq4qf5a9";
const NODE_IP: &str = "0.0.0.0:4133";
const BOOTSTRAP: [&str; 6] = [
    "164.92.111.59:4133",
    "159.223.204.96:4133",
    "167.71.219.176:4133",
    "157.245.205.209:4133",
    "134.122.95.106:4133",
    "161.35.24.55:4133",
];

pub async fn run_node_prover() {
    Node::new_prover(
        SocketAddr::from_str(NODE_IP).unwrap(),
        PrivateKey::<CurrentNetwork>::from_str(PRIVATE_KEY).unwrap(),
        &[
            SocketAddr::from_str("164.92.111.59:4133").unwrap(),
            SocketAddr::from_str("159.223.204.96:4133").unwrap(),
            SocketAddr::from_str("167.71.219.176:4133").unwrap(),
            SocketAddr::from_str("157.245.205.209:4133").unwrap(),
            SocketAddr::from_str("134.122.95.106:4133").unwrap(),
            SocketAddr::from_str("161.35.24.55:4133").unwrap(),
            SocketAddr::from_str("138.68.103.139:4133").unwrap(),
            SocketAddr::from_str("207.154.215.49:4133").unwrap(),
            SocketAddr::from_str("46.101.114.158:4133").unwrap(),
            SocketAddr::from_str("138.197.190.94:4133").unwrap(),
        ],
        None,
    )
    .await
    .unwrap();
}
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

fn get_last_nosuiffict_prove() {}

fn submit_prove() {}

fn share_worker() {}

fn get_difficult() {}

fn verity_prove() {}
