use anyhow::Result;
use async_graphql::SimpleObject;
use score::{EPOCH_LENGTH, block::header::EpochMark};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Epoch {
    id: i32,
    block: i32,
    entropy: String,
    tickets_entropy: String,
    validators: Vec<String>,
    validators_bandersnatches: Vec<String>,
    blocks: i32,
    tickets: i32,
    preimages: i32,
    preimages_size: i32,
    guarantees: i32,
    assurances: i32,
}

impl Epoch {
    pub async fn _list(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM epoches ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn get(pool: &PgPool, id: i32) -> Result<Self> {
        let data = query_as!(Self, "SELECT * FROM epoches WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    pub async fn get_by_block(pool: &PgPool, block: i32) -> Result<Self> {
        let data = query_as!(Self, "SELECT * FROM epoches WHERE block = $1", block)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, epoch: &EpochMark) -> Result<i32> {
        let entropy = hex::encode(epoch.entropy);
        let tickets_entropy = hex::encode(epoch.tickets_entropy);
        // save validator, and use ed25519 as the primary key
        let mut validators = vec![];
        let mut validators_bandersnatches = vec![];
        for validator in epoch.validators {
            validators.push(hex::encode(validator.ed25519));
            validators_bandersnatches.push(hex::encode(validator.bandersnatch));
        }

        let epoch_id = block / EPOCH_LENGTH as i32 + 1;
        if let Ok(_) = query_as!(Self, "SELECT * from epoches WHERE id = $1", epoch_id)
            .fetch_one(pool)
            .await
        {
            // update epoch TODO check epoch is valid
            query!(
                "UPDATE epoches SET block=$1,entropy=$2,tickets_entropy=$3,validators=$4,validators_bandersnatches=$5 WHERE id = $6",
                block,
                entropy,
                tickets_entropy,
                &validators,
                &validators_bandersnatches,
                epoch_id
            ).execute(pool).await?;
        } else {
            // insert epoch
            query!(
                "INSERT INTO epoches (id, block,entropy,tickets_entropy,validators,validators_bandersnatches) VALUES ($1,$2,$3,$4,$5,$6)",
                epoch_id,
                block,
                entropy,
                tickets_entropy,
                &validators,
                &validators_bandersnatches
            ).execute(pool).await?;
        }

        Ok(epoch_id)
    }

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
            "UPDATE epoches SET blocks=$1,tickets=$2,preimages=$3,preimages_size=$4,guarantees=$5,assurances=$6 WHERE id = $7",
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
