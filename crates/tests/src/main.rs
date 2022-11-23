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

#[tokio::main]
async fn main() {
    run_node_prover().await;
}
