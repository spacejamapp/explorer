use anyhow::{Result, anyhow};
use async_graphql::SimpleObject;
use score::service::{WorkExecResult, WorkResult as JamWorkResult};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct WorkResult {
    /// The key of the service
    id: i32,
    /// guarantee(work report) id
    guarantee: i32,
    /// The service id
    service: i32,
    /// The code hash
    code: String,
    /// The hash of payload
    payload: String,
    /// The accumulate gas
    gas: i64,
    /// The execute result (enum)
    result: String,
    /// Refine: gas used
    refine_gas: i64,
    /// Refine: the number of imports
    refine_imports: i32,
    /// Refine: the number of extrinsic
    refine_extrinsic_count: i32,
    /// Refine: the size of extrinsic
    refine_extrinsic_size: i32,
    /// Refine: the number of exports
    refine_exports: i32,
}

impl WorkResult {
    pub async fn _list_by_service(
        pool: &PgPool,
        service: i32,
        from: i64,
        to: i64,
    ) -> Result<Vec<Self>> {
        if to < from || to - from > 100 {
            return Err(anyhow!("No more than 100 rows in a single query"));
        }
        let offset = if from < 0 { 1 } else { from - 1 };

        let data = query_as!(
            Self,
            "SELECT * FROM work_results WHERE service = $1 ORDER BY id DESC LIMIT $2 OFFSET $3",
            service,
            to - from,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn _list_by_guarantee(pool: &PgPool, guarantee: i32) -> Result<Vec<Self>> {
        let data = query_as!(
            Self,
            "SELECT * FROM work_results WHERE guarantee = $1",
            guarantee,
        )
        .fetch_all(pool)
        .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, guarantee: i32, result: &JamWorkResult) -> Result<()> {
        let service = result.service_id as i32;
        let code = hex::encode(result.code_hash);
        let payload = hex::encode(result.payload_hash);
        let gas = result.accumulate_gas as i64;
        let wresult = match &result.result {
            WorkExecResult::Ok(data) => format!("Ok({})", hex::encode(data)),
            WorkExecResult::OutOfGas => "Out of gas".to_owned(),
            WorkExecResult::Panic => "Panic".to_owned(),
            WorkExecResult::BadCode => "Bad code".to_owned(),
            WorkExecResult::CodeOversize => "Code oversize".to_owned(),
        };
        let refine_gas = result.refine_load.gas_used as i64;
        let refine_imports = result.refine_load.imports as i32;
        let refine_extrinsic_count = result.refine_load.extrinsic_count as i32;
        let refine_extrinsic_size = result.refine_load.extrinsic_size as i32;
        let refine_exports = result.refine_load.exports as i32;

        query!(
            "INSERT INTO work_results (guarantee,service,code,payload,gas,result,refine_gas,refine_imports,refine_extrinsic_count,refine_extrinsic_size,refine_exports) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            guarantee,
            service,
            code,
            payload,
            gas,
            wresult,
            refine_gas,
            refine_imports,
            refine_extrinsic_count,
            refine_extrinsic_size,
            refine_exports
        )
            .execute(pool)
            .await?;

        Ok(())
    }
}
