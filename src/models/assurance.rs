use crate::models::hex;
use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::AvailAssurance;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Assurance {
    id: i32,
    block: i32,
    anchor: String,
    bitfield: String,
    validator_index: i32,
    signature: String,
}

impl Assurance {
    /// Count total assurances in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM assurances")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM assurances WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, assurance: &AvailAssurance) -> Result<()> {
        let anchor = hex(assurance.anchor);
        let bitfield = hex(assurance.bitfield);
        let validator_index = assurance.validator_index as i32;
        let signature = hex(assurance.signature);

        query!(
            "INSERT INTO assurances (block,anchor,bitfield,validator_index,signature) VALUES ($1,$2,$3,$4,$5)",
            block,
            anchor,
            bitfield,
            validator_index,
            signature,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
