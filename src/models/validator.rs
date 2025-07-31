use crate::{
    Manager,
    models::{Epoch, Header, try_hex},
};
use anyhow::Result;
use async_graphql::{
    ComplexObject, Context, Result as GraphqlResult, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Validator {
    pub id: i32,
    ed25519: String,
    bandersnatch: String,
    name: String,
    details: String,
    software: String,
    ip: String,
    website: String,
    scores: i32,
}

#[ComplexObject]
impl Validator {
    /// List this validator's all epochs data (DESC)
    pub async fn epochs(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> GraphqlResult<Connection<String, EpochValidator, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let data = EpochValidator::list_by_validator(pool, self.id, limit, cursor).await?;
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

    /// List this validator's all anchor blocks (DESC)
    pub async fn blocks(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10, validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(desc = "Cursor for pagination")] after: Option<String>,
    ) -> GraphqlResult<Connection<String, Header, EmptyFields, EmptyFields>> {
        let limit = first.unwrap_or(10).min(100);
        let cursor = after.unwrap_or_default().parse::<i32>().unwrap_or(0);
        let pool = &ctx.data::<Manager>()?.pg;

        let data = Header::list_by_author(pool, self.id, limit, cursor).await?;
        let has_prev_page = cursor != 0;
        let has_next_page = data.len() > limit as usize;
        let items = data.into_iter().take(limit as usize).collect::<Vec<_>>();

        let mut connection = Connection::new(has_prev_page, has_next_page);
        connection.edges = items
            .into_iter()
            .map(|item| Edge::new(item.slot.to_string(), item))
            .collect();
        Ok(connection)
    }

    /// Count the total blocks number
    pub async fn total_blocks(&self, ctx: &Context<'_>) -> GraphqlResult<i64> {
        let pool = &ctx.data::<Manager>()?.pg;
        let count = Header::count_by_author(pool, self.id).await?;
        Ok(count)
    }

    /// Count the total tickets number
    pub async fn total_tickets(&self, ctx: &Context<'_>) -> GraphqlResult<i64> {
        let pool = &ctx.data::<Manager>()?.pg;
        let count = EpochValidator::count_tickets_by_validator(pool, self.id).await?;
        Ok(count)
    }

    /// Count the total epochs number
    pub async fn total_epochs(&self, ctx: &Context<'_>) -> GraphqlResult<i64> {
        let pool = &ctx.data::<Manager>()?.pg;
        let count = EpochValidator::count_epochs_by_validator(pool, self.id).await?;
        Ok(count)
    }
}

impl Validator {
    /// List all services (ASC)
    pub async fn list(pool: &PgPool, limit: i32, cursor: i32) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM validators WHERE id>$1 ORDER BY id ASC LIMIT $2",
            cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn get(pool: &PgPool, id: i32) -> Result<Self> {
        let data = query_as!(Self, "SELECT * FROM validators WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    pub async fn get_by_ed25519(pool: &PgPool, hex: &str) -> Result<Self> {
        let hex = try_hex(hex)?;
        let data = query_as!(Self, "SELECT * FROM validators WHERE ed25519 = $1", hex)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(
        pool: &PgPool,
        epoch: i32,
        ed25519: &str,
        bandersnatch: &str,
        vindex: i32,
    ) -> Result<()> {
        // create validator
        let validator = if let Ok(validator) =
            query_as!(Self, "SELECT * FROM validators WHERE ed25519=$1", ed25519)
                .fetch_one(pool)
                .await
        {
            validator.id
        } else {
            query_scalar!(
                "INSERT INTO validators (ed25519,bandersnatch) VALUES ($1, $2) RETURNING id",
                ed25519,
                bandersnatch
            )
            .fetch_one(pool)
            .await?
        };

        // create validators with epoch
        EpochValidator::insert(pool, epoch, validator, vindex).await
    }
}

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct EpochValidator {
    pub id: i32,
    epoch_id: i32,
    validator_id: i32,
    vindex: i32,
    blocks: i32,
    tickets: i32,
    preimages: i32,
    guarantees: i32,
    assurances: i32,
}

#[ComplexObject]
impl EpochValidator {
    /// Get the Epoch
    pub async fn epoch(&self, ctx: &Context<'_>) -> GraphqlResult<Epoch> {
        let pool = &ctx.data::<Manager>()?.pg;
        Ok(Epoch::get(pool, self.epoch_id).await?)
    }

    /// Get the Validator
    pub async fn validator(&self, ctx: &Context<'_>) -> GraphqlResult<Validator> {
        let pool = &ctx.data::<Manager>()?.pg;
        Ok(Validator::get(pool, self.validator_id).await?)
    }
}

impl EpochValidator {
    /// List all validators in the epoch (ASC)
    pub async fn list_by_epoch(
        pool: &PgPool,
        epoch: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM epochs_validators WHERE epoch_id=$1 AND id>$2 ORDER BY id ASC LIMIT $3",
            epoch,
            cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    /// list all validator's epoch statistics (DESC)
    pub async fn list_by_validator(
        pool: &PgPool,
        validator: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let fixed_cursor = if cursor == 0 { i32::MAX } else { cursor };
        let data = query_as!(
            Self,
            "SELECT * FROM epochs_validators WHERE validator_id=$1 AND id<$2 ORDER BY id DESC LIMIT $3",
            validator,
            fixed_cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    /// count the validator's epochs
    pub async fn count_epochs_by_validator(pool: &PgPool, validator: i32) -> Result<i64> {
        let count = query_scalar!(
            "SELECT COUNT(id) FROM epochs_validators WHERE validator_id=$1",
            validator
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok(count)
    }

    /// count the validator's tickets
    pub async fn count_tickets_by_validator(pool: &PgPool, validator: i32) -> Result<i64> {
        let count = query_scalar!(
            "SELECT SUM(tickets) FROM epochs_validators WHERE validator_id=$1",
            validator
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok(count)
    }

    pub async fn insert(pool: &PgPool, epoch: i32, validator: i32, vindex: i32) -> Result<()> {
        if query_as!(
            Self,
            "SELECT * FROM epochs_validators WHERE epoch_id=$1 AND validator_id=$2",
            epoch,
            validator
        )
        .fetch_one(pool)
        .await
        .is_ok()
        {
            tracing::warn!("already had epochs_validators, epoch: {epoch}, validator: {validator}");
            return Ok(());
        }

        query!(
            "INSERT INTO epochs_validators (epoch_id,validator_id,vindex,blocks,tickets,preimages,guarantees,assurances) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
            epoch,
            validator,
            vindex,
            0,
            0,
            0,
            0,
            0
        )
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn new_block(
        pool: &PgPool,
        epoch: i32,
        vindex: i32,
        tickets: i32,
        preimages: i32,
        guarantees: i32,
        assurances: i32,
    ) -> Result<i32> {
        if let Ok(validators) = query_as!(
            Self,
            "SELECT * FROM epochs_validators WHERE epoch_id=$1",
            epoch
        )
        .fetch_all(pool)
        .await
        {
            for v in validators {
                if v.vindex == vindex {
                    // do update
                    query!(
                        "UPDATE epochs_validators SET blocks=$1,tickets=$2,preimages=$3,guarantees=$4,assurances=$5 WHERE id=$6",
                        v.blocks + 1,
                        v.tickets + tickets,
                        v.preimages + preimages,
                        v.guarantees + guarantees,
                        v.assurances + assurances,
                        v.id,
                    )
                        .execute(pool)
                        .await?;

                    return Ok(v.validator_id);
                }
            }
        }

        tracing::warn!("missed epochs_validators, epoch: {epoch}, vindex: {vindex}");
        Ok(0)
    }
}
