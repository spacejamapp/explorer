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
    /// Count total tickets in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM tickets")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM tickets WHERE block=$1", block)
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
