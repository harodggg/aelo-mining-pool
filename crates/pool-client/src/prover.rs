use std::sync::Arc;

// 1,接受块。
// 2，接受难度。
// 3，计算难度。
// 4，提交解决。
// 5， 不运行全节点，不连接其他网络。
// 6，把随机的nonce 改为固定的nonce
use anyhow::Result;
use simple_log::{info, log::warn};
use snarkvm::prelude::{
    Address, Block, CoinbasePuzzle, EpochChallenge, Network, PrivateKey, Testnet3, ViewKey,
};

const PRIVATE_KEY: &str = "APrivateKey1zkp7rs3Ls2qGjUKiTeEgP5DrLpowzXKgZ59uk4aGqWaNvev";
const VIEW_KEY: &str = "AViewKey1eq6impGbR8JoGvmU45gSWR1KBMH1uVoLmLp9hK65LqG4";
const ADDRESS: &str = "aleo1n3dx8azjks2vlnluyelxvzys5cd3tn3jqz4m82ln9g6uy4dsd5fq4qf5a9";

pub enum CoinbaseState {
    Start,
    Runing,
    Stop,
}

// todo! 改为多线程
pub fn try_coinbase_solution<N: Network>(
    from_nonce: u64,
    to_nonce: u64,
    address: Address<N>,
    last_target: u64,
    last_block: u64,
    epoch_challenge: &EpochChallenge<N>,
    state: CoinbaseState,
) {
    let puzzle: CoinbasePuzzle<N> = CoinbasePuzzle::<N>::load().unwrap();
    for nonce in from_nonce..to_nonce {
        match state {
            CoinbaseState::Start => {
                info! {"Start New Coinbase Solution"}
                puzzle
                    .prove(epoch_challenge, address, nonce, Some(last_block))
                    .unwrap();
            }
            CoinbaseState::Runing => {
                info! {"Computing Coinbase Solution,the nonce is {}",nonce}
            }
            CoinbaseState::Stop => {
                info! {"Stopping Coinbase"}
                break;
            }
        }
    }
}
