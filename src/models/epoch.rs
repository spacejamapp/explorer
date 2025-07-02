use anyhow::Result;
use async_graphql::{
    ComplexObject, Context, Result as GraphqlResult, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
};
use score::{EPOCH_LENGTH, block::header::EpochMark};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    Manager,
    models::{Core, Validator, hex},
};

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Epoch {
    id: i32,
    block: i32,
    entropy: String,
    tickets_entropy: String,
    validators_ed25519: Vec<String>,
    validators_bandersnatches: Vec<String>,
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
    ) -> GraphqlResult<Connection<String, Validator, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;
        let data = Validator::list_by_epoch(pool, self.id, limit, cursor).await?;
        let items = data.into_iter().take(limit as usize).collect::<Vec<_>>();

        let has_next_page = items.len() > limit as usize;
        let mut connection = Connection::new(false, has_next_page);
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
    ) -> GraphqlResult<Connection<String, Core, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;
        let data = Core::list_by_epoch(pool, self.id, limit, cursor).await?;
        let items = data.into_iter().take(limit as usize).collect::<Vec<_>>();

        let has_next_page = items.len() > limit as usize;
        let mut connection = Connection::new(false, has_next_page);
        connection.edges = items
            .into_iter()
            .map(|item| Edge::new(item.id.to_string(), item))
            .collect();
        Ok(connection)
    }
}

impl Epoch {
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
    #[allow(dead_code)]
    pub async fn insert(pool: &PgPool, block: i32, epoch: &EpochMark) -> Result<i32> {
        let entropy = hex(epoch.entropy);
        let tickets_entropy = hex(epoch.tickets_entropy);
        // save validator, and use ed25519 as the primary key
        let mut validators_ed25519 = vec![];
        let mut validators_bandersnatches = vec![];
        for validator in epoch.validators {
            validators_ed25519.push(hex(validator.ed25519));
            validators_bandersnatches.push(hex(validator.bandersnatch));
        }

        let epoch_id = block / EPOCH_LENGTH as i32 + 1;
        if query_as!(Self, "SELECT * from epochs WHERE id = $1", epoch_id)
            .fetch_one(pool)
            .await
            .is_ok()
        {
            // update epoch TODO check epoch is valid
            query!(
                "UPDATE epochs SET block=$1,entropy=$2,tickets_entropy=$3,validators_ed25519=$4,validators_bandersnatches=$5 WHERE id = $6",
                block,
                entropy,
                tickets_entropy,
                &validators_ed25519,
                &validators_bandersnatches,
                epoch_id
            ).execute(pool).await?;
        } else {
            // insert epoch
            query!(
                "INSERT INTO epochs (id, block,entropy,tickets_entropy,validators_ed25519,validators_bandersnatches) VALUES ($1,$2,$3,$4,$5,$6)",
                epoch_id,
                block,
                entropy,
                tickets_entropy,
                &validators_ed25519,
                &validators_bandersnatches
            ).execute(pool).await?;
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
