use anyhow::{Result, anyhow};
use async_graphql::SimpleObject;
use score::block::Header as JamHeader;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Header {
    pub slot: i32,
    pub hash: String,
    pub parent: String,
    pub parent_state_root: String,
    pub extrinsic_hash: String,
    pub extrinsic_works: i32,
    pub author_index: i32,
    pub entropy_source: String,
    pub seal: String,
    pub offenders_mark: Vec<String>,
    pub current_epoch: i32,
}

impl Header {
    pub async fn list(pool: &PgPool, from: i64, to: i64) -> Result<Vec<Self>> {
        if to < from || to - from > 100 {
            return Err(anyhow!("No more than 100 rows in a single query"));
        }
        let offset = if from < 0 { 1 } else { from - 1 };

        let data = query_as!(
            Self,
            "SELECT * FROM headers ORDER BY slot DESC LIMIT $1 OFFSET $2",
            to - from,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn get(pool: &PgPool, slot: i32) -> Result<Self> {
        let data = query_as!(Self, "SELECT * FROM headers WHERE slot=$1", slot)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(
        pool: &PgPool,
        slot: i32,
        extrinsic_works: i32,
        current_epoch: i32,
        header: &JamHeader,
    ) -> Result<()> {
        // save the block(header)
        let block_hash = hex::encode(header.hash()?);
        let parent = hex::encode(header.parent);
        let parent_state_root = hex::encode(header.parent_state_root);
        let extrinsic_hash = hex::encode(header.extrinsic_hash);
        let author_index = header.author_index;
        let entroy_source = hex::encode(header.entropy_source);
        let seal = hex::encode(header.seal);

        // FIXME if need save the offenders for validators?
        let offenders_mark = header
            .offenders_mark
            .iter()
            .map(|v| hex::encode(v))
            .collect::<Vec<String>>();

        query!(
            "INSERT INTO headers (slot,hash,parent,parent_state_root,extrinsic_hash,extrinsic_works,author_index,entropy_source,seal,offenders_mark,current_epoch) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
                slot,
                block_hash,
                parent,
                parent_state_root,
                extrinsic_hash,
                extrinsic_works,
                author_index as i32,
                entroy_source,
                seal,
                &offenders_mark,
                current_epoch,
            )
            .execute(pool)
            .await?;

        Ok(())
    }
}
