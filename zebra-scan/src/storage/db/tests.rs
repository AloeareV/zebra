//! General scanner database tests.

use std::sync::Arc;

use zebra_chain::{
    block::{Block, Height},
    parameters::Network::{self, *},
    serialization::ZcashDeserializeInto,
};
use zebra_state::TransactionIndex;

use crate::{
    storage::Storage,
    tests::{FAKE_SAPLING_VIEWING_KEY, ZECPAGES_SAPLING_VIEWING_KEY},
    Config,
};

#[cfg(test)]
mod snapshot;

/// Returns an empty `Storage` suitable for testing.
pub fn new_test_storage(network: Network) -> Storage {
    Storage::new(&Config::ephemeral(), network, false)
}

/// Add fake keys to `storage` for testing purposes.
pub fn add_fake_keys(storage: &mut Storage) {
    // Snapshot a birthday that is automatically set to activation height
    storage.add_sapling_key(&ZECPAGES_SAPLING_VIEWING_KEY.to_string(), None);
    // Snapshot a birthday above activation height
    storage.add_sapling_key(&FAKE_SAPLING_VIEWING_KEY.to_string(), Height(1_000_000));
}

/// Add fake results to `storage` for testing purposes.
pub fn add_fake_results(storage: &mut Storage, network: Network, height: Height) {
    let blocks = match network {
        Mainnet => &*zebra_test::vectors::CONTINUOUS_MAINNET_BLOCKS,
        Testnet => &*zebra_test::vectors::CONTINUOUS_TESTNET_BLOCKS,
    };

    let block: Arc<Block> = blocks
        .get(&height.0)
        .expect("block height has test data")
        .zcash_deserialize_into()
        .expect("test data deserializes");

    // Fake results from the first few blocks
    storage.add_sapling_results(
        &ZECPAGES_SAPLING_VIEWING_KEY.to_string(),
        height,
        block
            .transactions
            .iter()
            .enumerate()
            .map(|(index, tx)| (TransactionIndex::from_usize(index), tx.hash().into()))
            .collect(),
    );
}
