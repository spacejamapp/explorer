use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::TicketEnvelope;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::hex;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Envelope {
    id: i32,
    block: i32,
    attempt: i16,
    signature: String,
}

impl Envelope {
    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM envelopes WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, envelope: &TicketEnvelope) -> Result<()> {
        let attempt = envelope.attempt as i16;
        let signature = hex(envelope.signature);

        query!(
            "INSERT INTO envelopes (block,attempt,signature) VALUES ($1,$2,$3)",
            block,
            attempt,
            signature,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
