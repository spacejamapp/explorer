use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::TicketBody;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Ticket {
    id: i32,
    block: i32,
    ticket_id: String,
    attempt: i16,
}

impl Ticket {
    pub async fn list(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM tickets ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, ticket: &TicketBody) -> Result<()> {
        let ticket_id = hex::encode(ticket.id);
        query!(
            "INSERT INTO tickets (block,ticket_id,attempt) VALUES ($1,$2,$3)",
            block,
            ticket_id,
            ticket.attempt as i16,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
