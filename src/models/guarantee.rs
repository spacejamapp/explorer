use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::ReportGuarantee;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Guarantee {
    id: i32,
    block: i32,
    report: String,
    slot: i32,
    // [1:xxxx, 2:xxxx, 3:xxxx]
    signatures: Vec<String>,
}

impl Guarantee {
    pub async fn _list(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM guarantees ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, guarantee: &ReportGuarantee) -> Result<()> {
        let package_hash = hex::encode(&guarantee.report.spec.hash);
        let signatures = guarantee
            .signatures
            .iter()
            .map(|sig| format!("{}:{}", sig.validator_index, hex::encode(&sig.signature)))
            .collect::<Vec<String>>();

        // TODO save work report

        query!(
            "INSERT INTO guarantees (block,report,slot,signatures) VALUES ($1,$2,$3,$4)",
            block,
            package_hash,
            guarantee.slot as i32,
            &signatures,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
