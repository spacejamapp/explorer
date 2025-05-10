//! SpaceJam runtime hook for jamscan

use runtime::storage::KVStorage;
use spacejam::storage::Sled;
use sqlx::PgPool;

use crate::models::Block;

/// Spacejam runtime hook for jamscan
pub struct JamScanHook(PgPool, Sled);

impl JamScanHook {
    /// Create a new JamScanHook
    pub fn new(pool: PgPool, storage: Sled) -> Self {
        Self(pool, storage)
    }
}

impl runtime::Hook for JamScanHook {
    async fn on_finalized_block(&self, block: score::Block) -> anyhow::Result<()> {
        println!("block: {}", block.header.slot);
        // save the block
        let epoch = Block::insert(&self.0, &block).await?;
        println!("epoch: {}", epoch);

        // fetch current statistics from storage
        match self.1.get(score::state::key::STATISTICS) {
            Ok(Some(statistics)) => {
                println!("get statistics");
            }
            Ok(None) => {
                println!("no statistics");
            }
            Err(err) => {
                println!("Get statistics from storage error: {err}");
            }
        }

        Ok(())
    }
}
