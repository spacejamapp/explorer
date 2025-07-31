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

    // mock epoch for next generate
    for epoch in 10000..10020i32 {
        sqlx::query!(
            "INSERT INTO epochs (id, block,entropy,tickets_entropy) VALUES ($1,$2,$3,$4)",
            epoch,
            epoch,
            "0xthisismockedepoch0000000000000000000000000000000000000000000info",
            "0xthisismockedepoch00000000000000000000000000000000000000000ticket",
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    // mock core history (vindex (8, 9), with epochs (10000 - 10020))
    for vindex in 8..10i32 {
        for epoch in 10000..10020i32 {
            let v1 = Sha256::digest(epoch.to_be_bytes())[0];
            let v2 = (v1 as f64 / 255.0) * (1.25 - 0.75) + 0.75; // (0.75; ~ 1.25)
            let extrinsic_count = (epoch as f64 * v2) as i32;
            sqlx::query!(
                "INSERT INTO epochs_cores (epoch_id,vindex,gas_used,imports,extrinsic_count,extrinsic_size,exports,bundle_size,da_load,popularity) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
                epoch,
                vindex,
                epoch as i64 * 10, // gas_used
                epoch / 100, // imports
                extrinsic_count,
                extrinsic_count * 100, // extrinsic size
                extrinsic_count / 100, // exports
                (epoch as f32 * 1.1) as i32,           // bundle_size
                epoch as i64 * 1000,   // da_load
                epoch as i64 / 100     // popularity
            ).execute(&pool).await.unwrap();
        }
    }

    // mock validator history (validators(11-20), with epochs (10000 - 10020))
    for vindex in 11..21i32 {
        let r = Sha256::digest(vindex.to_be_bytes());
        let ed25519 = format!("0x{}", hex::encode(r));
        let bandersnatch = format!(
            "0x{}",
            hex::encode(Sha256::digest((vindex * 10).to_be_bytes()))
        );

        let validator: i32 = sqlx::query_scalar!(
            "INSERT INTO validators (ed25519,bandersnatch,name,details,software,ip,website,scores) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id",
            ed25519,
            bandersnatch,
            format!("Spacejam-{}", vindex),
            "This is spacejam official node",
            format!("spacejam v{}", r[0] % 3),
            format!("{}.{}.{}.{}", r[0], r[1], r[2], r[3]), // ip
            "https://spacejam.app",
            vindex, // scores
        )
            .fetch_one(&pool)
            .await.unwrap();

        for epoch in 10000..10020i32 {
            let vindex = vindex - 10;
            let v1 = Sha256::digest(vindex.to_be_bytes())[0];
            let blocks = ((v1 as f32 / 255.0) * 100f32) as i32; // (1 ~ 100)

            sqlx::query!(
                "INSERT INTO epochs_validators (epoch_id,validator_id,vindex,blocks,tickets,preimages,guarantees,assurances) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
                epoch,
                validator,
                vindex,
                blocks, // blocks
                blocks * 10, // tickets
                blocks / 10, // preimages
                blocks * 100, // guarantees
                blocks / 20  // assurances
            )
                .execute(&pool)
                .await.unwrap();
        }
    }
}
