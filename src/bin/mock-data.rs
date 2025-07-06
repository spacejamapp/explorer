//! Mock services data

use clap::Parser;
use sha2::{Digest, Sha256};

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

    println!("Start generating mock data...");

    // mock services
    for i in 1..=25i64 {
        let code = format!("0x{}", hex::encode(Sha256::digest(i.to_be_bytes())));
        sqlx::query!(
            "INSERT INTO services (id,code,balance,accumulate,transfer,total,items) VALUES ($1,$2,$3,$4,$5,$6,$7)",
            i as i32,
            code,     // code
            i * 100,  // balance
            i * 10,   // accumulate
            i * 10,   // transfer
            i * 1000,  // total
            i as i32 * 100, // items
        )
            .execute(&pool)
            .await.unwrap();
    }
    println!("Generated mock services");

    // mock work report
}
