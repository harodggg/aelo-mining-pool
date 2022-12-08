use anyhow::Result;
use indexmap::IndexMap;
use simple_log::error;
use snarkos_node_ledger::Ledger;
use snarkvm::prelude::{ConsensusStorage, Network};

use crate::{BlockLocators, NUM_RECENTS, CHECKPOINT_INTERVAL};


/// Returns the block locators for the given ledger.
pub fn get_block_locators<N: Network, C: ConsensusStorage<N>>(
    ledger: &Ledger<N, C>,
) -> Result<BlockLocators<N>> {
    // Retrieve the latest height.
    let latest_height = ledger.latest_height();

    // Initialize the recents map.
    let mut recents = IndexMap::with_capacity(NUM_RECENTS);

    // Retrieve the recent block hashes.
    for height in latest_height.saturating_sub((NUM_RECENTS - 1) as u32)..=latest_height {
        recents.insert(height, ledger.get_hash(height)?);
    }

    // Initialize the checkpoints map.
    let mut checkpoints =
        IndexMap::with_capacity((latest_height % CHECKPOINT_INTERVAL).try_into()?);

    // Retrieve the checkpoint block hashes.
    for height in (0..=latest_height).step_by(CHECKPOINT_INTERVAL as usize) {
        checkpoints.insert(height, ledger.get_hash(height)?);
    }

    // Construct the block locators.
    Ok(BlockLocators::new(recents, checkpoints))
}

/// A helper to log instructions to recover.
pub fn log_clean_error(dev: Option<u16>) {
    match dev {
        Some(id) => {
            error!("Storage corruption detected! Run `snarkos clean --dev {id}` to reset storage")
        }
        None => error!("Storage corruption detected! Run `snarkos clean` to reset storage"),
    }
}
