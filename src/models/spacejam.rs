use anyhow::Result;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

const BLOCKS_NAME: &str = "blocks";

#[derive(Serialize, Deserialize)]
pub struct SpaceJam {
    name: String,
    value: String,
}

impl SpaceJam {
    pub async fn blocks(pool: &PgPool) -> Result<(i32, i64)> {
        let data = query_as!(Self, "SELECT * FROM spacejams WHERE name = $1", BLOCKS_NAME)
            .fetch_one(pool)
            .await?;
        let mut info = data.value.split(",");
        let finalized = info.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
        let extrinsic = info.next().unwrap_or("0").parse::<i64>().unwrap_or(0);

        Ok((finalized, extrinsic))
    }

    pub async fn set_blocks(pool: &PgPool, finalized: i32, extrinsic: i32) -> Result<()> {
        if let Ok((_, old_extrinsic)) = Self::blocks(pool).await {
            let value = format!("{},{}", finalized, extrinsic as i64 + old_extrinsic);
            query!(
                "UPDATE spacejams SET value = $1 WHERE name = $2",
                value,
                BLOCKS_NAME,
            )
            .execute(pool)
            .await?;
        } else {
            let value = format!("{},{}", finalized, extrinsic);
            query!(
                "INSERT INTO spacejams (name,value) VALUES ($1,$2)",
                BLOCKS_NAME,
                value,
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct GraphqlSpaceJam {
    pub finalized: i32,
    pub extrinsic: i64,
}

impl GraphqlSpaceJam {
    pub async fn get(pool: &PgPool) -> Result<Self> {
        let (finalized, extrinsic) = SpaceJam::blocks(pool).await?;

        Ok(GraphqlSpaceJam {
            finalized,
            extrinsic,
        })
    }
}
