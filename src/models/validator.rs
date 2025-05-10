use anyhow::{Result, anyhow};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Validator {
    id: i32,
    epoch: i32,
    vindex: i32,
    blocks: i32,
    tickets: i32,
    preimages: i32,
    guarantees: i32,
    assurances: i32,
}

impl Validator {
    pub async fn list_by_epoch(pool: &PgPool, epoch: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM validators WHERE epoch=$1", epoch)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn list_by_vindex(
        pool: &PgPool,
        vindex: i32,
        from: i64,
        to: i64,
    ) -> Result<Vec<Self>> {
        if to < from || to - from > 100 {
            return Err(anyhow!("No more than 100 rows in a single query"));
        }
        let offset = if from < 0 { 1 } else { from - 1 };

        let data = query_as!(
            Self,
            "SELECT * FROM validators WHERE vindex=$1 ORDER BY id DESC LIMIT $2 OFFSET $3",
            vindex,
            to - from,
            offset
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
