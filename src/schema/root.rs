//! The query root for jamscan

use crate::{
    Manager,
    manager::Spacejam,
    models::{Block, Core, Epoch, Header, Validator},
};
use async_graphql::{Context, Object, Result};

/// Query root for jamscan
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn headers(&self, ctx: &Context<'_>, from: i64, to: i64) -> Result<Vec<Header>> {
        let pool = &ctx.data::<Manager>()?.pg;
        let data = Header::list(pool, from, to).await?;
        Ok(data)
    }

    /// Get the raw block data from the database.
    #[graphql(name = "blockRaw")]
    async fn block_raw(&self, ctx: &Context<'_>, slot: i32) -> Result<String> {
        let pool = &ctx.data::<Manager>()?.pg;
        let raw = Block::raw(pool, slot).await?;
        Ok(raw)
    }

    /// Get the block data from the database.
    async fn block(&self, ctx: &Context<'_>, slot: i32) -> Result<Block> {
        let pool = &ctx.data::<Manager>()?.pg;
        let block = Block::get(pool, slot).await?;
        Ok(block)
    }

    async fn epoch(&self, ctx: &Context<'_>, id: i32) -> Result<Epoch> {
        let pool = &ctx.data::<Manager>()?.pg;
        let epoch = Epoch::get(pool, id).await?;
        Ok(epoch)
    }

    async fn validators(&self, ctx: &Context<'_>, epoch: i32) -> Result<Vec<Validator>> {
        let pool = &ctx.data::<Manager>()?.pg;
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
        let pool = &ctx.data::<Manager>()?.pg;
        let block = Validator::list_by_vindex(pool, index, from, to).await?;
        Ok(block)
    }

    async fn core(&self, ctx: &Context<'_>, index: i32, from: i64, to: i64) -> Result<Vec<Core>> {
        let pool = &ctx.data::<Manager>()?.pg;
        let cores = Core::list_by_index(pool, index, from, to).await?;
        Ok(cores)
    }

    /// Get the spacejam cache
    async fn spacejam(&self, ctx: &Context<'_>) -> Result<Spacejam> {
        Ok(ctx.data::<Manager>()?.spacejam.read().await.clone())
    }
}
