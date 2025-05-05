//! The query root for jamscan

use async_graphql::{Context, Object};
use sqlx::PgPool;

/// Query root for jamscan
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn headers(&self, ctx: &Context<'_>) -> Vec<String> {
        let pool = ctx.data::<PgPool>().unwrap();
        let headers: Vec<String> = sqlx::query_scalar!("SELECT hash FROM headers")
            .fetch_all(pool)
            .await
            .unwrap();
        headers
    }
}
