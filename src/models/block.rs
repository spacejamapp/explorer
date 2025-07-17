use crate::{
    Manager,
    models::{
        Assurance, DisputeCulprit, DisputeFault, DisputeVerdict, Envelope, Epoch, EpochValidator,
        Guarantee, Header, Preimage, Ticket, Validator,
    },
};
use anyhow::Result;
use async_graphql::{Context, Object, Result as GraphqlResult, SimpleObject};
use score::{EPOCH_LENGTH, block::Block as JamBlock};
use serde::{Deserialize, Serialize};
use spacejson::Json;
use sqlx::PgPool;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct BlockHeader {
    slot: i32,
    hash: String,
    parent: String,
    parent_state_root: String,
    extrinsic_hash: String,
    extrinsic_count: i32,
    author_index: i32,
    entropy_source: String,
    seal: String,
    offenders_mark: Vec<String>,
    epoch_mark: Option<Epoch>,
    tickets_mark: Vec<Ticket>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct BlockExtrinsic {
    tickets: Vec<Envelope>,
    preimages: Vec<Preimage>,
    guarantees: Vec<Guarantee>,
    assurances: Vec<Assurance>,
    disputes: Dispute,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Dispute {
    verdicts: Vec<DisputeVerdict>,
    culprits: Vec<DisputeCulprit>,
    faults: Vec<DisputeFault>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub slot: i32,
    pub anchor_id: i32,
}

#[Object]
impl Block {
    async fn anchor(&self, ctx: &Context<'_>) -> GraphqlResult<Validator> {
        let pool = &ctx.data::<Manager>()?.pg;
        Ok(Validator::get(pool, self.anchor_id).await?)
    }

    async fn header(&self, ctx: &Context<'_>) -> GraphqlResult<BlockHeader> {
        let slot = self.slot;
        let pool = &ctx.data::<Manager>()?.pg;

        // load header
        let header = Header::get(pool, slot).await?;
        let epoch = Epoch::get_by_block(pool, slot).await.ok();
        let tickets = Ticket::list_by_block(pool, slot).await?;
        let block_header = BlockHeader {
            slot: header.slot,
            hash: header.hash,
            parent: header.parent,
            parent_state_root: header.parent_state_root,
            extrinsic_hash: header.extrinsic_hash,
            extrinsic_count: header.extrinsic_count,
            author_index: header.author_index,
            entropy_source: header.entropy_source,
            seal: header.seal,
            offenders_mark: header.offenders_mark,
            epoch_mark: epoch,
            tickets_mark: tickets,
        };

        Ok(block_header)
    }

    async fn extrinsic(&self, ctx: &Context<'_>) -> GraphqlResult<BlockExtrinsic> {
        let slot = self.slot;
        let pool = &ctx.data::<Manager>()?.pg;

        // load extrinsic
        let envelopes = Envelope::list_by_block(pool, slot).await?;
        let preimages = Preimage::list_by_block(pool, slot).await?;
        let guarantees = Guarantee::list_by_block(pool, slot).await?;
        let assurances = Assurance::list_by_block(pool, slot).await?;
        let verdicts = DisputeVerdict::list_by_block(pool, slot).await?;
        let culprits = DisputeCulprit::list_by_block(pool, slot).await?;
        let faults = DisputeFault::list_by_block(pool, slot).await?;
        let disputes = Dispute {
            verdicts,
            culprits,
            faults,
        };
        let extrinsic = BlockExtrinsic {
            tickets: envelopes,
            preimages,
            guarantees,
            assurances,
            disputes,
        };

        Ok(extrinsic)
    }
}

impl Block {
    /// Count total blocks in the database
    pub async fn count(pool: &PgPool) -> Result<i64> {
        let count = query_scalar!("SELECT COUNT(*) FROM blocks")
            .fetch_one(pool)
            .await?
            .unwrap_or(0);
        Ok(count)
    }

    /// Get the block data from the database.
    pub async fn get(pool: &PgPool, slot: i32) -> Result<Self> {
        let block = query_as!(
            Self,
            "SELECT slot,anchor_id FROM blocks WHERE slot=$1",
            slot
        )
        .fetch_one(pool)
        .await?;

        Ok(block)
    }

    /// Get the raw block data from the database.
    pub async fn raw(pool: &PgPool, slot: i32) -> Result<String> {
        let raw = query_scalar!("SELECT raw FROM blocks WHERE slot=$1", slot)
            .fetch_one(pool)
            .await?;
        Ok(raw)
    }

    /// list all validator's anchor blocks (DESC)
    pub async fn list_by_anchor(
        pool: &PgPool,
        anchor: i32,
        limit: i32,
        cursor: i32,
    ) -> Result<Vec<Self>> {
        let fixed_cursor = if cursor == 0 { i32::MAX } else { cursor };
        let data = query_as!(
            Self,
            "SELECT slot,anchor_id FROM blocks WHERE anchor_id=$1 AND slot<$2 ORDER BY slot DESC LIMIT $3",
            anchor,
            fixed_cursor,
            limit as i64 + 1
        )
            .fetch_all(pool)
            .await?;

        Ok(data)
    }

    pub async fn insert(pool: &PgPool, block: &JamBlock) -> Result<i32> {
        let raw = serde_json::to_string(&block.clone().to_json()).unwrap_or("".to_owned());
        let slot = block.header.slot as i32;

        // save epoch
        let epoch_id = if let Some(epoch) = &block.header.epoch_mark {
            Epoch::insert(pool, slot, epoch).await?
        } else {
            slot / EPOCH_LENGTH as i32
        };

        // save tickets
        if let Some(tickets) = &block.header.tickets_mark {
            for ticket in tickets {
                Ticket::insert(pool, slot, ticket).await?;
            }
        }

        // FIXME if this ticket envelop is related to header's tickets
        let tickets_num = block.extrinsic.tickets.len() as i32;
        for envelope in block.extrinsic.tickets.iter() {
            Envelope::insert(pool, slot, envelope).await?;
        }

        // save preimages
        let preimages_num = block.extrinsic.preimages.len() as i32;
        for preimage in block.extrinsic.preimages.iter() {
            Preimage::insert(pool, slot, preimage).await?;
        }

        // save guarantee
        let mut extrinsic_count = 0i32;
        let guarantees_num = block.extrinsic.guarantees.len() as i32;
        for guarantee in block.extrinsic.guarantees.iter() {
            let num = Guarantee::insert(pool, slot, guarantee).await?;
            extrinsic_count += num;
        }

        // save assurance
        let assurances_num = block.extrinsic.assurances.len() as i32;
        for assurance in block.extrinsic.assurances.iter() {
            Assurance::insert(pool, slot, assurance).await?;
        }

        // Disputes
        // save verdict
        for verdict in block.extrinsic.disputes.verdicts.iter() {
            DisputeVerdict::insert(pool, slot, verdict).await?;
        }

        // save culprit
        for culprit in block.extrinsic.disputes.culprits.iter() {
            DisputeCulprit::insert(pool, slot, culprit).await?;
        }

        // save fault
        for fault in block.extrinsic.disputes.faults.iter() {
            DisputeFault::insert(pool, slot, fault).await?;
        }

        // save header
        Header::insert(pool, slot, extrinsic_count, epoch_id, &block.header).await?;

        // save validators
        let anchor = EpochValidator::new_block(
            pool,
            epoch_id,
            block.header.author_index as i32,
            tickets_num,
            preimages_num,
            guarantees_num,
            assurances_num,
        )
        .await?;

        // save raw block
        query!(
            "INSERT INTO blocks (slot,anchor_id,raw) VALUES ($1,$2,$3)",
            slot,
            anchor,
            raw
        )
        .execute(pool)
        .await?;

        Ok(epoch_id)
    }
}
