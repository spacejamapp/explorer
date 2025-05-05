//! Stores header data in postgres

use async_graphql::{EmptyMutation, EmptySubscription};
use jadex::{Config, service};
use jamscan::{JamScanHook, schema::QueryRoot};
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config {
        postgres: "postgres://postgres:postgres@localhost/jamscan".into(),
        data: "jamscan.db".into(),
        genesis: None,
        graphql: "0.0.0.0:3000".parse()?,
        quic: "0.0.0.0:6888".parse()?,
    };

    let pool = PgPool::connect(&config.postgres).await?;
    let hook = JamScanHook::from(pool.clone());

    tracing::info!("Running graphql server at {}", config.graphql);
    tokio::select! {
        r = service::node::dev(&config, hook.clone()) => r,
        r = service::graphql::start(
            QueryRoot,
            EmptyMutation,
            EmptySubscription,
            pool,
            config.graphql,
        ) => r,
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
