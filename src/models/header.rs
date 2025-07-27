use crate::{
    Manager,
    models::{Validator, hex},
};
use anyhow::Result;
use async_graphql::{ComplexObject, Context, Result as GraphqlResult, SimpleObject};
use score::block::Header as JamHeader;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Header {
    pub slot: i32,
    pub hash: String,
    pub parent: String,
    pub parent_state_root: String,
    pub extrinsic_hash: String,
    pub extrinsic_count: i32,
    pub author_index: i32,
    pub entropy_source: String,
    pub seal: String,
    pub offenders_mark: Vec<String>,
    pub current_epoch: i32,
    pub author_id: i32,
}

#[ComplexObject]
impl Header {
    async fn author(&self, ctx: &Context<'_>) -> GraphqlResult<Validator> {
        let pool = &ctx.data::<Manager>()?.pg;
        Ok(Validator::get(pool, self.author_id).await?)
    }
}

impl Header {
    /// List all headers (DESC)
    pub async fn list(pool: &PgPool, limit: i32, cursor: i32) -> Result<Vec<Self>> {
        let fixed_cursor = if cursor == 0 { i32::MAX } else { cursor };
        let data = query_as!(
            Self,
            "SELECT * FROM headers WHERE slot < $1 ORDER BY slot DESC LIMIT $2",
            fixed_cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    /// list all validator's anchoring blocks (DESC)
    pub async fn list_by_author(
        pool: &PgPool,
        author: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let fixed_cursor = if cursor == 0 { i32::MAX } else { cursor };
        let data = query_as!(
            Self,
            "SELECT * FROM headers WHERE author_id=$1 AND slot<$2 ORDER BY slot DESC LIMIT $3",
            author,
            fixed_cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    /// NOTE: this will not being used, consider remove it.
    pub async fn get(pool: &PgPool, slot: i32) -> Result<Self> {
        let data = query_as!(Self, "SELECT * FROM headers WHERE slot=$1", slot)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(
        pool: &PgPool,
        slot: i32,
        extrinsic_count: i32,
        current_epoch: i32,
        author_id: i32,
        header: &JamHeader,
    ) -> Result<()> {
        // save the block(header)
        let block_hash = hex(header.hash()?);
        let parent = hex(header.parent);
        let parent_state_root = hex(header.parent_state_root);
        let extrinsic_hash = hex(header.extrinsic_hash);
        let author_index = header.author_index as i32;
        let entropy_source = hex(header.entropy_source);
        let seal = hex(header.seal);

        // FIXME if need save the offenders for validators?
        let offenders_mark = header
            .offenders_mark
            .iter()
            .map(hex)
            .collect::<Vec<String>>();

        query!(
            "INSERT INTO headers (slot,hash,parent,parent_state_root,extrinsic_hash,extrinsic_count,author_index,entropy_source,seal,offenders_mark,current_epoch,author_id) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
                slot,
                block_hash,
                parent,
                parent_state_root,
                extrinsic_hash,
                extrinsic_count,
                author_index,
                entropy_source,
                seal,
                &offenders_mark,
                current_epoch,
                author_id
            )
            .execute(pool)
            .await?;

        Ok(())
    }
}
