//! The query root for jamscan

use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

use crate::models::{Block, GraphqlSpaceJam, Header};

/// Query root for jamscan
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn blocks(&self, ctx: &Context<'_>, from: i64, to: i64) -> Result<Vec<Header>> {
        let pool = ctx.data::<PgPool>()?;
        let data = Header::list(pool, from, to).await?;
        Ok(data)
    }

    async fn block(&self, ctx: &Context<'_>, slot: i32) -> Result<Block> {
        let pool = ctx.data::<PgPool>()?;
        let block = Block::get(pool, slot).await?;
        Ok(block)
    }

    async fn spacejam(&self, ctx: &Context<'_>) -> Result<GraphqlSpaceJam> {
        let pool = ctx.data::<PgPool>()?;
        let data = GraphqlSpaceJam::get(pool).await?;
        Ok(data)
    }
}
