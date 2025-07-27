use crate::{Manager, models::Epoch};
use anyhow::Result;
use async_graphql::{ComplexObject, Context, Result as GraphqlResult, SimpleObject};
use score::statistic::CoreActivityRecord;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct EpochCore {
    pub id: i32,
    epoch_id: i32,
    vindex: i32,
    gas_used: i64,
    imports: i32,
    extrinsic_count: i32,
    extrinsic_size: i32,
    exports: i32,
    bundle_size: i32,
    da_load: i64,
    popularity: i64,
}

#[ComplexObject]
impl EpochCore {
    /// Get the Epoch
    pub async fn epoch(&self, ctx: &Context<'_>) -> GraphqlResult<Epoch> {
        let pool = &ctx.data::<Manager>()?.pg;
        Ok(Epoch::get(pool, self.epoch_id).await?)
    }
}

impl EpochCore {
    /// List all cores in the epoch (ASC)
    pub async fn list_by_epoch(
        pool: &PgPool,
        epoch: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM epochs_cores WHERE epoch_id=$1 AND id>$2 ORDER BY id ASC LIMIT $3",
            epoch,
            cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    /// list all core's epoch statistics (DESC)
    pub async fn list_by_index(
        pool: &PgPool,
        index: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let fixed_cursor = if cursor == 0 { i32::MAX } else { cursor };
        let data = query_as!(
            Self,
            "SELECT * FROM epochs_cores WHERE vindex=$1 AND id<$2 ORDER BY id DESC LIMIT $3",
            index,
            fixed_cursor,
            limit as i64 + 1
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn statistic(
        pool: &PgPool,
        epoch: i32,
        vindex: i32,
        record: &CoreActivityRecord,
    ) -> Result<()> {
        if let Ok(c) = query_as!(
            Self,
            "SELECT * from epochs_cores WHERE vindex = $1 AND epoch_id = $2",
            vindex,
            epoch
        )
        .fetch_one(pool)
        .await
        {
            query!(
                "UPDATE epochs_cores SET gas_used=$1,imports=$2,extrinsic_count=$3,extrinsic_size=$4,exports=$5,bundle_size=$6,da_load=$7,popularity=$8 WHERE id = $9",
                c.gas_used + record.gas_used as i64,
                c.imports + record.imports as i32,
                c.extrinsic_count + record.extrinsic_count as i32,
                c.extrinsic_size + record.extrinsic_size as i32,
                c.exports + record.exports as i32,
                c.bundle_size + record.bundle_size as i32,
                c.da_load + record.da_load as i64,
                c.popularity + record.popularity as i64,
                c.id
            ).execute(pool).await?;
        } else {
            // insert epoch
            query!(
                "INSERT INTO epochs_cores (epoch_id,vindex,gas_used,imports,extrinsic_count,extrinsic_size,exports,bundle_size,da_load,popularity) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
                epoch,
                vindex,
                record.gas_used as i64,
                record.imports as i32,
                record.extrinsic_count as i32,
                record.extrinsic_size as i32,
                record.exports as i32,
                record.bundle_size as i32,
                record.da_load as i64,
                record.popularity as i64,
            ).execute(pool).await?;
        }

        Ok(())
    }
}
