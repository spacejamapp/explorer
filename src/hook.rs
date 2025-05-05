//! SpaceJam runtime hook for jamscan

use sqlx::Executor;
use sqlx::PgPool;

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
        self.0
            .execute(sqlx::query!(
                "INSERT INTO headers (hash) VALUES ($1)",
                hex::encode(block.header.hash()?)
            ))
            .await?;
        Ok(())
    }
}
