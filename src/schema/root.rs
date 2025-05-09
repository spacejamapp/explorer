//! The query root for jamscan

use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

use crate::models::{Block, Epoch, Header, Ticket};

/// Query root for jamscan
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn headers(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] offset: i64,
        #[graphql(default = 10)] limit: i64,
    ) -> Result<Vec<Header>> {
        let pool = ctx.data::<PgPool>()?;
        let data = Header::list(pool, offset, limit).await?;
        Ok(data)
    }

    async fn tickets(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] offset: i64,
        #[graphql(default = 10)] limit: i64,
    ) -> Result<Vec<Ticket>> {
        let pool = ctx.data::<PgPool>()?;
        let data = Ticket::list(pool, offset, limit).await?;
        Ok(data)
    }

    async fn epoches(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] offset: i64,
        #[graphql(default = 10)] limit: i64,
    ) -> Result<Vec<Epoch>> {
        let pool = ctx.data::<PgPool>()?;
        let data = Epoch::list(pool, offset, limit).await?;
        Ok(data)
    }
}
