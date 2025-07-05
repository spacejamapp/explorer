//! JAM scan service

use async_graphql::{EmptyMutation, EmptySubscription};
use clap::{ArgAction, CommandFactory, Parser};
use jadex::{
    config::{Cors, Graphql, Node},
    service,
};
use jamscan::{Manager, schema::QueryRoot};
use std::net::SocketAddr;
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

    /// Verbosity level
    #[arg(short, long, default_value = "0", action = ArgAction::Count)]
    verbose: u8,

    #[command(flatten)]
    node: Node,
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

    let graphql = Graphql {
        cors: Cors::default(),
        graphql: SocketAddr::from(([0, 0, 0, 0], args.graphql_port)),
    };

    let manager = Manager::new(&args.postgres, &args.redis).await?;
    tracing::info!("Running graphql server at {}", graphql.graphql);
    tokio::select! {
        r = service::node::start(args.node, manager.clone()) => r,
        r = service::graphql::start(
            QueryRoot,
            EmptyMutation,
            EmptySubscription,
            manager.clone(),
            &graphql,
        ) => r,
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
