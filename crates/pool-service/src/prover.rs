use anyhow::Result;
use core::str::FromStr;
use snarkos_node::Node;
use snarkvm::prelude::PrivateKey;
use snarkvm::prelude::{Network, Testnet3};

use std::net::SocketAddr;

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

pub async fn run_prover() {
    Node::new_prover(
        SocketAddr::from_str(NODE_IP).unwrap(),
        PrivateKey::<CurrentNetwork>::from_str(PRIVATE_KEY).unwrap(),
        &[
            SocketAddr::from_str(BOOTSTRAP[1]).unwrap(),
            SocketAddr::from_str(BOOTSTRAP[2]).unwrap(),
            SocketAddr::from_str(BOOTSTRAP[3]).unwrap(),
            SocketAddr::from_str(BOOTSTRAP[4]).unwrap(),
            SocketAddr::from_str(BOOTSTRAP[5]).unwrap(),
        ],
    )
    .await
    .unwrap();
}

fn get_last_nosuiffict_prove() {}

fn submit_prove() {}

fn share_worker() {}

fn get_difficult() {}

fn verity_prove() {}
