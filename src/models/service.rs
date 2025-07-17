use crate::{
    Manager,
    models::{Preimage, WorkResult, hex},
};
use anyhow::Result;
use async_graphql::{
    ComplexObject, Context, Result as GraphqlResult, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
};
use score::{ServiceId, service::ServiceData};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Service {
    /// The key of the service
    pub id: i32,
    /// The code hash of the service account (c)
    code: String,
    /// The balance of the service account (b)
    balance: i64,
    /// The accumulate gas of the service account (g)
    accumulate: i64,
    /// The minimum required for the on transfer entry-point (m)
    transfer: i64,
    /// The total number of octets used in storage (o)
    total: i64,
    /// The number of items in storage (i)
    items: i32,
}

#[ComplexObject]
impl Service {
    async fn preimages(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> GraphqlResult<Connection<String, Preimage, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let data = Preimage::list_by_service(pool, self.id, limit, cursor).await?;
        let has_prev_page = cursor == 0;
        let has_next_page = data.len() > limit as usize;
        let items = data.into_iter().take(limit as usize).collect::<Vec<_>>();

        let mut connection = Connection::new(has_prev_page, has_next_page);
        connection.edges = items
            .into_iter()
            .map(|item| Edge::new(item.id.to_string(), item))
            .collect();
        Ok(connection)
    }

    async fn works(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> GraphqlResult<Connection<String, WorkResult, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let data = WorkResult::list_by_service(pool, self.id, limit, cursor).await?;
        let has_prev_page = cursor == 0;
        let has_next_page = data.len() > limit as usize;
        let items = data.into_iter().take(limit as usize).collect::<Vec<_>>();

        let mut connection = Connection::new(has_prev_page, has_next_page);
        connection.edges = items
            .into_iter()
            .map(|item| Edge::new(item.id.to_string(), item))
            .collect();
        Ok(connection)
    }
}

impl Service {
    /// Count total services in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM services")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    /// List all services (DESC)
    pub async fn list(pool: &PgPool, limit: i32, cursor: i32) -> Result<Vec<Self>> {
        let fixed_cursor = if cursor == 0 { i32::MAX } else { cursor };
        let data = query_as!(
            Self,
            "SELECT * FROM services WHERE id<$1 ORDER BY id DESC LIMIT $2",
            fixed_cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn get(pool: &PgPool, id: i32) -> Result<Self> {
        Ok(query_as!(Self, "SELECT * from services WHERE id = $1", id)
            .fetch_one(pool)
            .await?)
    }

    pub async fn insert(pool: &PgPool, sid: ServiceId, data: &ServiceData) -> Result<()> {
        let id = sid as i32;
        let code = hex(data.code);

        // update service
        if let Ok(s) = Self::get(pool, id).await {
            query!(
                "UPDATE services SET code=$1,balance=$2,accumulate=$3,transfer=$4,total=$5,items=$6 WHERE id=$7",
                code,
                data.balance as i64,
                data.accumulate as i64,
                data.transfer as i64,
                data.total as i64,
                data.items as i32,
                s.id
            ).execute(pool).await?;
        } else {
            query!(
                "INSERT INTO services (id,code,balance,accumulate,transfer,total,items) VALUES ($1,$2,$3,$4,$5,$6,$7)",
                id,
                code,
                data.balance as i64,
                data.accumulate as i64,
                data.transfer as i64,
                data.total as i64,
                data.items as i32
            )
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}
