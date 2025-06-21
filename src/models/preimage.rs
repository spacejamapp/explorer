use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::Preimage as JamPreimage;
use serde::{Deserialize, Serialize};
use spacejam_crypto::blake2b;
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Preimage {
    id: i32,
    block: i32,
    requester: i32,
    hash: String,
    blob: Vec<u8>,
}

impl Preimage {
    /// Count total preimages in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM preimages")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM preimages WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, preimage: &JamPreimage) -> Result<()> {
        let image_hash = hex::encode(blake2b(&preimage.blob));

        query!(
            "INSERT INTO preimages (block,requester,hash,blob) VALUES ($1,$2,$3,$4)",
            block,
            preimage.requester as i32,
            image_hash,
            preimage.blob
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
