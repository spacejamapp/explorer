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
    pub async fn _list(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM preimages ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
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
