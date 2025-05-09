use anyhow::Result;
use async_graphql::SimpleObject;
use score::block::Header as JamHeader;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Header {
    slot: i32,
    hash: String,
    parent: String,
    parent_state_root: String,
    extrinsic_hash: String,
    author_index: i32,
    entropy_source: String,
    seal: String,
    offenders_mark: Vec<String>,
}

impl Header {
    pub async fn list(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM headers ORDER BY slot DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, slot: i32, header: &JamHeader) -> Result<()> {
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
            "INSERT INTO headers (slot,hash,parent,parent_state_root,extrinsic_hash,author_index,entropy_source,seal,offenders_mark) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
                slot,
                block_hash,
                parent,
                parent_state_root,
                extrinsic_hash,
                author_index as i32,
                entroy_source,
                seal,
                &offenders_mark,
            )
            .execute(pool)
            .await?;

        Ok(())
    }
}
