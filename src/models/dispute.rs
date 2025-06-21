use anyhow::Result;
use async_graphql::SimpleObject;
use score::extrinsic::{Culprit, Fault, Verdict};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM dispute_verdicts WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, verdict: &Verdict) -> Result<()> {
        let target = hex::encode(verdict.target);
        let age = verdict.age as i32;
        let signatures = verdict
            .votes
            .iter()
            .map(|v| format!("{}:{}:{}", v.vote, v.index, hex::encode(v.signature)))
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
    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM dispute_culprits WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, culprit: &Culprit) -> Result<()> {
        let target = hex::encode(culprit.target);
        let key = hex::encode(culprit.key);
        let signature = hex::encode(culprit.signature);

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
    pub async fn list_by_block(pool: &PgPool, block: i32) -> Result<Vec<Self>> {
        let data = query_as!(Self, "SELECT * FROM dispute_faults WHERE block=$1", block)
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: i32, fault: &Fault) -> Result<()> {
        let target = hex::encode(fault.target);
        let vote = fault.vote;
        let key = hex::encode(fault.key);
        let signature = hex::encode(fault.signature);

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
