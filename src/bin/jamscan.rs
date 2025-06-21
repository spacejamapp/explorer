//! Stores header data in postgres

use async_graphql::{EmptyMutation, EmptySubscription};
use clap::{ArgAction, CommandFactory, Parser};
use jadex::{
    config::{Config, Cors, Graphql, Node},
    service,
};
use jamscan::{Manager, schema::QueryRoot};
use std::{net::SocketAddr, path::PathBuf};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    postgres: String,

    /// Redis URL
    #[arg(long, env = "REDIS_URL")]
    redis: String,

    /// Graphql service port
    #[arg(long, env = "GRAPHQL_PORT", default_value = "3000")]
    graphql_port: u16,

    /// Graphql service port
    #[arg(long, env = "QUIC_PORT", default_value = "6888")]
    quic_port: u16,

    /// Chain data path
    #[arg(long, env = "DATA_PATH", default_value = "jamscan_db")]
    data_path: PathBuf,

    /// Verbosity level
    #[arg(short, long, default_value = "0", action = ArgAction::Count)]
    verbose: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let args = Command::parse();
    let name = Command::command().get_name().to_string();
    let env = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(match args.verbose {
        0 => format!("{name}=info,jamdex=info"),
        1 => format!("{name}=debug,jamdex=debug,spacejam=debug"),
        2 => "debug".into(),
        _ => "trace".into(),
    }));
    tracing_subscriber::fmt().with_env_filter(env).init();

    let config = Config {
        node: Node {
            data: args.data_path,
            spec: None,
            quic: SocketAddr::from(([0, 0, 0, 0], args.quic_port)),
        },
        graphql: Graphql {
            cors: Cors::default(),
            graphql: SocketAddr::from(([0, 0, 0, 0], args.graphql_port)),
        },
    };

    let manager = Manager::new(&args.postgres, &args.redis).await?;
    tracing::info!("Running graphql server at {}", config.graphql.graphql);
    tokio::select! {
        r = service::node::dev(&config, manager.clone()) => r,
        r = service::graphql::start(
            QueryRoot,
            EmptyMutation,
            EmptySubscription,
            manager.clone(),
            &config.graphql,
        ) => r,
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
