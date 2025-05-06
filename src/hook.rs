//! SpaceJam runtime hook for jamscan

use sqlx::PgPool;

use crate::models::Header;

/// Spacejam runtime hook for jamscan
#[derive(Clone)]
pub struct JamScanHook(PgPool);

impl JamScanHook {
    /// Create a new JamScanHook
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

impl From<PgPool> for JamScanHook {
    fn from(pool: PgPool) -> Self {
        Self(pool)
    }
}

impl runtime::Hook for JamScanHook {
    async fn on_finalized_block(&self, block: score::Block) -> anyhow::Result<()> {
        // save the header
        Header::insert(&self.0, &block.header).await?;

        // save the body(extrinsic)

        Ok(())
    }
}
