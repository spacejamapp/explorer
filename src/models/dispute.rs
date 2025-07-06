//! TODO: introduce a single type with enum for all dispute related types

use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::{Culprit, Fault, Verdict};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::hex;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DisputeVerdict {
    id: i32,
    block: i32,
    target: String,
    age: i32,
    // [true:1:xxxx, false:2:xxxx, true:3:xxxx]
    votes: Vec<String>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DisputeCulprit {
    id: i32,
    block: i32,
    target: String,
    key: String,
    signature: String,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DisputeFault {
    id: i32,
    block: i32,
    target: String,
    vote: bool,
    key: String,
    signature: String,
}

impl DisputeVerdict {
    /// Count total verdicts in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM dispute_verdicts")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM dispute_verdicts WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, verdict: &Verdict) -> Result<()> {
        let target = hex(verdict.target);
        let age = verdict.age as i32;
        let signatures = verdict
            .votes
            .iter()
            .map(|v| format!("{}:{}:{}", v.vote, v.index, hex(v.signature)))
            .collect::<Vec<String>>();

        query!(
            "INSERT INTO dispute_verdicts (block,target,age,votes) VALUES ($1,$2,$3,$4)",
            block,
            target,
            age,
            &signatures,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl DisputeCulprit {
    /// Count total culprits in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM dispute_culprits")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM dispute_culprits WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, culprit: &Culprit) -> Result<()> {
        let target = hex(culprit.target);
        let key = hex(culprit.key);
        let signature = hex(culprit.signature);

        query!(
            "INSERT INTO dispute_culprits (block,target,key,signature) VALUES ($1,$2,$3,$4)",
            block,
            target,
            key,
            signature,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl DisputeFault {
    /// Count total faults in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM dispute_faults")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM dispute_faults WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, fault: &Fault) -> Result<()> {
        let target = hex(fault.target);
        let vote = fault.vote;
        let key = hex(fault.key);
        let signature = hex(fault.signature);

        query!(
            "INSERT INTO dispute_faults (block,target,vote,key,signature) VALUES ($1,$2,$3,$4,$5)",
            block,
            target,
            vote,
            key,
            signature,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
