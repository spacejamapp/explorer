//! The query root for jamscan

use crate::models::{Block, Core, Epoch, GraphqlSpaceJam, Header, Validator};
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

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

    async fn epoch(&self, ctx: &Context<'_>, id: i32) -> Result<Epoch> {
        let pool = ctx.data::<PgPool>()?;
        let epoch = Epoch::get(pool, id).await?;
        Ok(epoch)
    }

    async fn spacejam(&self, ctx: &Context<'_>) -> Result<GraphqlSpaceJam> {
        let pool = ctx.data::<PgPool>()?;
        let data = GraphqlSpaceJam::get(pool).await?;
        Ok(data)
    }

    async fn validators(&self, ctx: &Context<'_>, epoch: i32) -> Result<Vec<Validator>> {
        let pool = ctx.data::<PgPool>()?;
        let block = Validator::list_by_epoch(pool, epoch).await?;
        Ok(block)
    }

    async fn validator(
        &self,
        ctx: &Context<'_>,
        index: i32,
        from: i64,
        to: i64,
    ) -> Result<Vec<Validator>> {
        let pool = ctx.data::<PgPool>()?;
        let block = Validator::list_by_vindex(pool, index, from, to).await?;
        Ok(block)
    }

    async fn core(&self, ctx: &Context<'_>, index: i32, from: i64, to: i64) -> Result<Vec<Core>> {
        let pool = ctx.data::<PgPool>()?;
        let cores = Core::list_by_index(pool, index, from, to).await?;
        Ok(cores)
    }
}
