use crate::models::hex;
use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::Preimage as JamPreimage;
use serde::{Deserialize, Serialize};
use spacejam_crypto::blake2b;
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Preimage {
    pub id: i32,
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

    /// List preimages by block
    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM preimages WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    /// List preimages by service (DESC)
    pub async fn list_by_service(
        pool: &PgPool,
        service: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let fixed_cursor = if cursor == 0 { i32::MAX } else { cursor };
        let data = query_as!(
            Self,
            "SELECT * FROM preimages WHERE requester=$1 AND id<$2 ORDER BY id DESC LIMIT $3",
            service,
            fixed_cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, preimage: &JamPreimage) -> Result<()> {
        let image_hash = hex(blake2b(&preimage.blob));

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
