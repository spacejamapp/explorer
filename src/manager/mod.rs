//! Data manager

use anyhow::Result;
use redis::Client;
pub use spacejam::Spacejam;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

mod hook;
mod spacejam;

/// Data manager
#[derive(Clone)]
pub struct Manager {
    /// Postgres pool
    pub pg: PgPool,

    /// Redis client
    pub redis: Arc<Client>,

    /// Spacejam cache
    pub spacejam: Arc<RwLock<Spacejam>>,
}

impl Manager {
    /// Create a new manager
    ///
    /// on creating the manager, we calculate the global state of the chain
    /// and save it to the redis
    pub async fn new(pg: &str, redis: &str) -> Result<Self> {
        let pg = PgPool::connect(pg).await?;
        let redis = Arc::new(Client::open(redis)?);
        let spacejam = Arc::new(RwLock::new(Spacejam::init(&pg).await?));

        let this = Self {
            pg,
            redis,
            spacejam,
        };

        Ok(this)
    }

    /// On finalized block
    pub async fn on_finalized_block(&self, block: &score::Block) -> Result<()> {
        let mut spacejam = self.spacejam.write().await;
        spacejam.assurances += block.extrinsic.assurances.len() as i64;
        spacejam.guarantees += block.extrinsic.guarantees.len() as i64;
        spacejam.tickets += block.extrinsic.tickets.len() as i64;
        spacejam.preimages += block.extrinsic.preimages.len() as i64;
        spacejam.disputes_verdicts += block.extrinsic.disputes.verdicts.len() as i64;
        spacejam.disputes_culprits += block.extrinsic.disputes.culprits.len() as i64;
        spacejam.disputes_faults += block.extrinsic.disputes.faults.len() as i64;
        spacejam.blocks += 1;
        Ok(())
    }
}
