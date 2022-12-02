use crate::client::Prover;
use anyhow::Result;
use core::str::FromStr;
use snarkos_account::Account;
use snarkos_node::Node;
use snarkos_node_store::ConsensusDB;
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

pub async fn run_node_prover(dev: Option<u16>) -> Result<()> {
    Node::new_prover(
        SocketAddr::from_str(NODE_IP)?,
        PrivateKey::<CurrentNetwork>::from_str(PRIVATE_KEY)?,
        &[],
        dev,
    )
    .await?;
    Ok(())
}
pub async fn run_prover(dev: Option<u16>) -> Result<()> {
    Prover::<CurrentNetwork, ConsensusDB<CurrentNetwork>>::new(
        SocketAddr::from_str(NODE_IP)?,
        Account::<CurrentNetwork>::from_str(PRIVATE_KEY)?,
        &[],
        dev,
    )
    .await?;
    Ok(())
}

fn get_last_nosuiffict_prove() {}

fn submit_prove() {}

fn share_worker() {}

fn get_difficult() {}

fn verity_prove() {}
