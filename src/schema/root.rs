//! The query root for jamscan

use crate::{
    Manager,
    manager::Spacejam,
    models::{Block, Epoch, EpochCore, Header, Service, Validator},
};
use async_graphql::{
    Context, Object, Result,
    connection::{Connection, Edge, EmptyFields},
};

/// Query root for jamscan
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get the spacejam cache
    async fn spacejam(&self, ctx: &Context<'_>) -> Result<Spacejam> {
        Ok(ctx.data::<Manager>()?.spacejam.read().await.clone())
    }

    /// List headers by page
    async fn headers(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> Result<Connection<String, Header, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let headers = Header::list(pool, limit, cursor).await?;
        let items = headers.into_iter().take(limit as usize).collect::<Vec<_>>();

        let has_next_page = items.len() > limit as usize;
        let mut connection = Connection::new(false, has_next_page);
        connection.edges = items
            .into_iter()
            .map(|item| Edge::new(item.slot.to_string(), item))
            .collect();
        Ok(connection)
    }

    /// Get the raw block data from the database.
    #[graphql(name = "blockRaw")]
    async fn block_raw(&self, ctx: &Context<'_>, slot: i32) -> Result<String> {
        let pool = &ctx.data::<Manager>()?.pg;
        let raw = Block::raw(pool, slot).await?;
        Ok(raw)
    }

    /// Get the block data from the database.
    async fn block(&self, ctx: &Context<'_>, slot: i32) -> Result<Option<Block>> {
        let pool = &ctx.data::<Manager>()?.pg;
        let block = Block::get(pool, slot).await.ok();
        Ok(block)
    }

    /// Get the epoch by id
    async fn epoch(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Epoch>> {
        let pool = &ctx.data::<Manager>()?.pg;
        let epoch = Epoch::get(pool, id).await.ok();
        Ok(epoch)
    }

    /// Get the epoch by id
    async fn validator(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Validator>> {
        let pool = &ctx.data::<Manager>()?.pg;
        let validator = Validator::get(pool, id).await.ok();
        Ok(validator)
    }

    /// Get the core with all epoch statistics
    async fn core(
        &self,
        ctx: &Context<'_>,
        index: i32,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> Result<Connection<String, EpochCore, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let cores = EpochCore::list_by_index(pool, index, limit, cursor).await?;
        let items = cores.into_iter().take(limit as usize).collect::<Vec<_>>();

        let has_next_page = items.len() > limit as usize;
        let mut connection = Connection::new(false, has_next_page);
        connection.edges = items
            .into_iter()
            .map(|item| Edge::new(item.id.to_string(), item))
            .collect();
        Ok(connection)
    }

    /// List all services
    async fn services(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> Result<Connection<String, Service, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let services = Service::list(pool, limit, cursor).await?;
        let items = services
            .into_iter()
            .take(limit as usize)
            .collect::<Vec<_>>();

        let has_next_page = items.len() > limit as usize;
        let mut connection = Connection::new(false, has_next_page);
        connection.edges = items
            .into_iter()
            .map(|item| Edge::new(item.id.to_string(), item))
            .collect();
        Ok(connection)
    }

    /// Get the service
    async fn service(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Service>> {
        let pool = &ctx.data::<Manager>()?.pg;
        let service = Service::get(pool, id).await.ok();
        Ok(service)
    }
}
