//! Data manager

use anyhow::Result;
use redis::Client;
use sqlx::PgPool;
use std::sync::Arc;

mod hook;

/// Data manager
#[derive(Clone)]
pub struct Manager {
    /// Postgres pool
    pub pg: PgPool,

    /// Redis client
    pub redis: Arc<Client>,
}

impl Manager {
    /// Create a new manager
    pub async fn new(pg: &str, redis: &str) -> Result<Self> {
        Ok(Self {
            pg: PgPool::connect(pg).await?,
            redis: Arc::new(Client::open(redis)?),
        })
    }
}
