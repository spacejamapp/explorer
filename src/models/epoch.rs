use crate::{
    Manager,
    models::{EpochCore, EpochValidator, Validator, hex},
};
use anyhow::Result;
use async_graphql::{
    ComplexObject, Context, Result as GraphqlResult, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
};
use score::{EPOCH_LENGTH, block::header::EpochMark};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Epoch {
    pub id: i32,
    block: i32,
    entropy: String,
    tickets_entropy: String,
    blocks: i32,
    tickets: i32,
    preimages: i32,
    preimages_size: i32,
    guarantees: i32,
    assurances: i32,
}

#[ComplexObject]
impl Epoch {
    async fn validators(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> GraphqlResult<Connection<String, EpochValidator, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let data = EpochValidator::list_by_epoch(pool, self.id, limit, cursor).await?;
        let has_prev_page = cursor != 0;
        let has_next_page = data.len() > limit as usize;
        let items = data.into_iter().take(limit as usize).collect::<Vec<_>>();

        let mut connection = Connection::new(has_prev_page, has_next_page);
        connection.edges = items
            .into_iter()
            .map(|item| Edge::new(item.id.to_string(), item))
            .collect();
        Ok(connection)
    }

    async fn cores(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> GraphqlResult<Connection<String, EpochCore, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let data = EpochCore::list_by_epoch(pool, self.id, limit, cursor).await?;
        let has_prev_page = cursor != 0;
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

impl Epoch {
    pub async fn last(pool: &PgPool) -> Result<Self> {
        let data = query_as!(Self, "SELECT * FROM epochs ORDER BY id DESC limit 1")
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    pub async fn get(pool: &PgPool, id: i32) -> Result<Self> {
        let data = query_as!(Self, "SELECT * FROM epochs WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    pub async fn get_by_block(pool: &PgPool, block: i32) -> Result<Self> {
        let data = query_as!(Self, "SELECT * FROM epochs WHERE block = $1", block)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    /// FIXME: should accumulate the extrinsic count
    //#[allow(dead_code)]
    pub async fn insert(pool: &PgPool, block: i32, epoch: &EpochMark) -> Result<i32> {
        let entropy = hex(epoch.entropy);
        let tickets_entropy = hex(epoch.tickets_entropy);
        let epoch_id = block / EPOCH_LENGTH as i32 + 1;

        if query_as!(Self, "SELECT * from epochs WHERE id = $1", epoch_id)
            .fetch_one(pool)
            .await
            .is_ok()
        {
            // update epoch TODO check epoch is valid
            query!(
                "UPDATE epochs SET block=$1,entropy=$2,tickets_entropy=$3 WHERE id = $4",
                block,
                entropy,
                tickets_entropy,
                epoch_id
            )
            .execute(pool)
            .await?;
        } else {
            // insert epoch
            query!(
                "INSERT INTO epochs (id, block,entropy,tickets_entropy) VALUES ($1,$2,$3,$4)",
                epoch_id,
                block,
                entropy,
                tickets_entropy,
            )
            .execute(pool)
            .await?;
        }

        // try insert validator
        for (vindex, validator) in epoch.validators.iter().enumerate() {
            let _ = Validator::insert(
                pool,
                epoch_id,
                &hex(validator.ed25519),
                &hex(validator.bandersnatch),
                vindex as i32,
            )
            .await;
        }

        Ok(epoch_id)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn statistic(
        pool: &PgPool,
        id: i32,
        blocks: i32,
        tickets: i32,
        preimages: i32,
        preimages_size: i32,
        guarantees: i32,
        assurances: i32,
    ) -> Result<()> {
        query!(
            "UPDATE epochs SET blocks=$1,tickets=$2,preimages=$3,preimages_size=$4,guarantees=$5,assurances=$6 WHERE id = $7",
            blocks,
            tickets,
            preimages,
            preimages_size,
            guarantees,
            assurances,
            id
        ).execute(pool).await?;

        Ok(())
    }
}
