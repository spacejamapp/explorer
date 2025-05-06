use anyhow::Result;
use async_graphql::SimpleObject;
use score::block::header::EpochMark;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Epoch {
    id: i32,
    entropy: String,
    tickets_entropy: String,
    validators: Vec<String>,
    validators_bandersnatches: Vec<String>,
}

impl Epoch {
    pub async fn list(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<Self>> {
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

    pub async fn insert(pool: &PgPool, epoch: &EpochMark) -> Result<String> {
        let entropy = hex::encode(epoch.entropy);
        let tickets_entropy = hex::encode(epoch.tickets_entropy);

        // save validator, and use ed25519 as the primary key
        let mut validators = vec![];
        let mut validators_bandersnatches = vec![];
        for validator in epoch.validators {
            validators.push(hex::encode(validator.ed25519));
            validators_bandersnatches.push(hex::encode(validator.bandersnatch));
        }

        // insert epoch
        query!(
            "INSERT INTO epoches (entropy,tickets_entropy,validators,validators_bandersnatches) VALUES ($1,$2,$3,$4)",
            entropy,
            tickets_entropy,
            &validators,
            &validators_bandersnatches
        ).execute(pool).await?;

        Ok(entropy)
    }
}
