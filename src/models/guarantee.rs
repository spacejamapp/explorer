use crate::models::{WorkResult, hex};
use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::ReportGuarantee;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// ReportGuarantee (WorkReport)
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Guarantee {
    id: i32,
    block: i32,
    slot: i32,
    /// The signatures [1:xxxx, 2:xxxx, 3:xxxx]
    signatures: Vec<String>,
    /// The hash of the package spec
    spec: String,
    /// The core index of this
    core: i32,
    /// The authorizer hash
    authorizer_hash: String,
    /// The auth output
    auth_output: String,
    /// The auth gas used
    auth_gas: i64,
    // Vec<SegmentRootLookup>
    // RefineContext
}

impl Guarantee {
    /// Count total guarantees in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM guarantees")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM guarantees WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, guarantee: &ReportGuarantee) -> Result<i32> {
        let slot = guarantee.slot as i32;
        let signatures = guarantee
            .signatures
            .iter()
            .map(|sig| format!("{}:{}", sig.validator_index, hex(sig.signature)))
            .collect::<Vec<String>>();
        let spec = hex(guarantee.report.spec.hash);
        let core = guarantee.report.core_index as i32;
        let authorizer_hash = hex(guarantee.report.authorizer_hash);
        let auth_output = hex(&guarantee.report.auth_output);
        let auth_gas = guarantee.report.auth_gas_used as i64;

        let id = query_scalar!(
            "INSERT INTO guarantees (block,slot,signatures,spec,core,authorizer_hash,auth_output,auth_gas) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id",
            block,
            slot,
            &signatures,
            spec,
            core,
            authorizer_hash,
            auth_output,
            auth_gas
        )
            .fetch_one(pool)
            .await?;

        // save work result
        let num = guarantee.report.results.len() as i32;
        for r in guarantee.report.results.iter() {
            let _ = WorkResult::insert(pool, id, r).await;
        }

        Ok(num)
    }
}
