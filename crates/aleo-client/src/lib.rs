mod block_locator;
pub mod client;
mod helpers;
pub use block_locator::*;
pub mod prover;
pub mod version;

#[macro_use]
extern crate async_trait;

pub use snarkos_node_messages::NodeType;

use snarkos_account::Account;
use snarkos_node_store::ConsensusDB;
use snarkvm::prelude::{Address, Block, ConsensusMemory, Network, PrivateKey, ViewKey};

// 实现观察者模式。
struct block {}

pub trait Observer {
    fn update();
}

pub trait Subject {
    fn notifyObserver();
}
