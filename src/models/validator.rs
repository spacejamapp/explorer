use anyhow::Result;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Validator {
    pub id: i32,
    epoch: i32,
    vindex: i32,
    blocks: i32,
    tickets: i32,
    preimages: i32,
    guarantees: i32,
    assurances: i32,
}

impl Validator {
    /// List all validators in the epoch (ASC)
    pub async fn list_by_epoch(
        pool: &PgPool,
        epoch: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM validators WHERE epoch=$1 AND id>$2 ORDER BY id ASC LIMIT $3",
            epoch,
            cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    /// list all validator's epoch statistics (DESC)
    pub async fn list_by_index(
        pool: &PgPool,
        index: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let fixed_cursor = if cursor == 0 { i32::MAX } else { cursor };
        let data = query_as!(
            Self,
            "SELECT * FROM validators WHERE vindex=$1 AND id<$2 ORDER BY id DESC LIMIT $3",
            index,
            fixed_cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn new_block(
        pool: &PgPool,
        epoch: i32,
        vindex: i32,
        tickets: i32,
        preimages: i32,
        guarantees: i32,
        assurances: i32,
    ) -> Result<()> {
        if let Ok(validators) = query_as!(Self, "SELECT * FROM validators WHERE epoch=$1", epoch)
            .fetch_all(pool)
            .await
        {
            for v in validators {
                if v.vindex == vindex {
                    // do update
                    query!(
                        "UPDATE validators SET blocks=$1,tickets=$2,preimages=$3,guarantees=$4,assurances=$5 WHERE id=$6",
                        v.blocks + 1,
                        v.tickets + tickets,
                        v.preimages + preimages,
                        v.guarantees + guarantees,
                        v.assurances + assurances,
                        v.id,
                    )
                        .execute(pool)
                        .await?;

                    return Ok(());
                }
            }
        }

        query!(
            "INSERT INTO validators (epoch,vindex,blocks,tickets,preimages,guarantees,assurances) VALUES ($1,$2,$3,$4,$5,$6,$7)",
            epoch,
            vindex,
            1,
            tickets,
            preimages,
            guarantees,
            assurances
        )
            .execute(pool)
            .await?;

        Ok(())
    }
}
