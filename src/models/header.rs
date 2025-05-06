use anyhow::Result;
use async_graphql::SimpleObject;
use score::block::Header as BlockHeader;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::{Epoch, Ticket};

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Header {
    id: i32,
    hash: String,
    parent: String,
    parent_state_root: String,
    extrinsic_hash: String,
    slot: i32,
    epoch_mark: Option<String>,
    tickets_mark: Option<Vec<String>>,
    offenders_mark: Vec<String>,
    author_index: i32,
    entropy_source: String,
    seal: String,
}

impl Header {
    pub async fn list(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM headers ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, header: &BlockHeader) -> Result<()> {
        // save the header
        let block_hash = hex::encode(header.hash()?);
        let parent = hex::encode(header.parent);
        let parent_state_root = hex::encode(header.parent_state_root);
        let extrinsic_hash = hex::encode(header.extrinsic_hash);
        let slot = header.slot;
        let epoch_mark = if let Some(epoch) = &header.epoch_mark {
            let entropy = Epoch::insert(pool, epoch).await?;
            Some(entropy)
        } else {
            None
        };
        let tickets_mark = if let Some(tickets) = &header.tickets_mark {
            let mut saved_tickets = vec![];
            for ticket in tickets {
                let ticket_id = Ticket::insert(pool, ticket).await?;
                saved_tickets.push(ticket_id);
            }
            saved_tickets
        } else {
            vec![]
        };
        let offenders_mark = header
            .offenders_mark
            .iter()
            .map(|v| hex::encode(v))
            .collect::<Vec<String>>();
        let author_index = header.author_index;
        let entroy_source = hex::encode(header.entropy_source);
        let seal = hex::encode(header.seal);

        query!(
                "INSERT INTO headers (hash,parent,parent_state_root,extrinsic_hash,slot,epoch_mark,tickets_mark,offenders_mark,author_index,entropy_source,seal) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
                block_hash,
                parent,
                parent_state_root,
                extrinsic_hash,
                slot as i32,
                epoch_mark,
                &tickets_mark,
                &offenders_mark,
                author_index as i32,
                entroy_source,
                seal,
            )
            .execute(pool)
            .await?;

        Ok(())
    }
}
