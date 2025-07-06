//! Dump graphql SDL

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use clap::Parser;
use jamscan::schema::QueryRoot;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    postgres: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let args = Command::parse();
    let pool = sqlx::PgPool::connect(&args.postgres).await.unwrap();

    println!("Start dumping the graphql SDL file...");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish();

    let sdl = schema.sdl();

    std::fs::write("schema.graphql", &sdl).expect("Failed to write schema to file");
    println!("Generated GraphQL Schema: schema.graphql");
}
