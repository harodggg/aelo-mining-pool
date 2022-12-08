use crate::client::Prover;
use anyhow::Result;
use core::str::FromStr;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use snarkos_account::Account;
use snarkos_node_store::ConsensusDB;
use snarkvm::{
    prelude::{Network, PrivateKey},
    synthesizer::{Block, ConsensusMemory, ConsensusStore, VM},
};

use std::net::SocketAddr;

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

// pub async fn run_node_prover(dev: Option<u16>) -> Result<()> {
//     let mut rng = ChaChaRng::seed_from_u64(1234567890u64);
//     let beacon_private_key = PrivateKey::<N>::new(&mut rng)?;
//     // Initialize a new VM.
//     let vm = VM::from(ConsensusStore::<N, ConsensusMemory<N>>::open(None)?)?;
//     // Initialize the genesis block.
//     let genesis = Block::genesis(&vm, &beacon_private_key, &mut rng)?;
//     Node::new_prover(
//         SocketAddr::from_str(NODE_IP)?,
//         PrivateKey::<CurrentNetwork>::from_str(PRIVATE_KEY)?,
//         &[],
//         dev,
//     )
//     .await?;
//     Ok(())
// }
pub async fn run_prover<N: Network>(dev: u16) -> Result<()> {
    let trusted_peers: &mut Vec<SocketAddr> = &mut vec![];
    for i in 0..dev {
        trusted_peers.push(SocketAddr::from_str(&format!("127.0.0.1:{}", 4130 + i))?);
    }
    // Set the node IP to `4130 + dev`.
    let node = SocketAddr::from_str(&format!("0.0.0.0:{}", 4130 + dev))?;

    // Initialize an (insecure) fixed RNG.
    let mut rng = ChaChaRng::seed_from_u64(1234567890u64);
    // Initialize the beacon private key.
    let beacon_private_key = PrivateKey::<N>::new(&mut rng)?;
    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<N, ConsensusMemory<N>>::open(None)?)?;
    // Initialize the genesis block.
    let genesis = Block::genesis(&vm, &beacon_private_key, &mut rng)?;
    // Initialize the genesis block.
    Prover::<N, ConsensusDB<N>>::new(
        node,
        Account::<N>::from_str(&beacon_private_key.to_string())?,
        &[],
        genesis,
        Some(dev),
    )
    .await?;
    Ok(())
}

fn get_last_nosuiffict_prove() {}

fn submit_prove() {}

fn share_worker() {}

fn get_difficult() {}

fn verity_prove() {}
