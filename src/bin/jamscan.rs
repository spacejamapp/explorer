//! Stores header data in postgres

use async_graphql::{EmptyMutation, EmptySubscription};
use clap::Parser;
use jadex::{Config, service};
use jamscan::{JamScanHook, schema::QueryRoot};
use sqlx::PgPool;
use std::{net::SocketAddr, path::PathBuf};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    database: String,

    /// Graphql service port
    #[arg(long, env = "GRAPHQL_PORT", default_value = "3000")]
    graphql_port: u16,

    /// Graphql service port
    #[arg(long, env = "QUIC_PORT", default_value = "6888")]
    quic_port: u16,

    /// Chain data path
    #[arg(long, env = "DATA_PATH", default_value = "jamscan.db")]
    data_path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Command::parse();

    let config = Config {
        postgres: args.database,
        data: args.data_path,
        genesis: None,
        graphql: SocketAddr::from(([0, 0, 0, 0], args.graphql_port)),
        quic: SocketAddr::from(([0, 0, 0, 0], args.quic_port)),
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
