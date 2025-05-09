use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::TicketEnvelope;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Envelope {
    id: i32,
    block: i32,
    attempt: i16,
    signature: String,
}

impl Envelope {
    pub async fn _list(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM envelopes ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, envelope: &TicketEnvelope) -> Result<()> {
        let attempt = envelope.attempt as i16;
        let signature = hex::encode(&envelope.signature);

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
