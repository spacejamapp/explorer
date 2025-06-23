use anyhow::{Result, anyhow};
use async_graphql::{ComplexObject, Context, Result as GraphqlResult, SimpleObject};
use score::{ServiceId, service::ServiceData};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    Manager,
    models::{Preimage, WorkResult},
};

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Service {
    /// The key of the service
    id: i32,
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
    async fn preimages(&self, ctx: &Context<'_>) -> GraphqlResult<Vec<Preimage>> {
        let pool = &ctx.data::<Manager>()?.pg;
        let preimages = Preimage::list_by_service(pool, self.id).await?;
        Ok(preimages)
    }

    async fn works(&self, ctx: &Context<'_>) -> GraphqlResult<Vec<WorkResult>> {
        let pool = &ctx.data::<Manager>()?.pg;
        let works = WorkResult::list_by_service(pool, self.id, 1, 100).await?;
        Ok(works)
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

    pub async fn list(pool: &PgPool, from: i64, to: i64) -> Result<Vec<Self>> {
        if to < from || to - from > 100 {
            return Err(anyhow!("No more than 100 rows in a single query"));
        }
        let offset = if from < 0 { 1 } else { from - 1 };

        let data = query_as!(
            Self,
            "SELECT * FROM services ORDER BY id DESC LIMIT $1 OFFSET $2",
            to - from,
            offset
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
        let code = hex::encode(data.code);

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
