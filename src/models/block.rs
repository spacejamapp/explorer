use anyhow::Result;
use async_graphql::SimpleObject;
use score::block::Block as JamBlock;
use serde::{Deserialize, Serialize};
use spacejson::Json;
use sqlx::PgPool;

use super::{
    Assurance, DisputeCulprit, DisputeFault, DisputeVerdict, Envelope, Epoch, Guarantee, Header,
    Preimage, Ticket,
};

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Block {
    slot: i32,
    raw: String,
    // header: graphql
    //   slot: i32,
    //   hash: String,
    //   parent: String,
    //   parent_state_root: String,
    //   extrinsic_hash: String,
    //   author_index: i32,
    //   entropy_source: String,
    //   seal: String,
    //   epoch_mark: Option<Epoch>,
    //   tickets_mark: Vec<Ticket>,
    //   offenders_mark: Vec<OffenderValidator>,
    // extrinsic: graphql
    //   tickets: Vec<Ticket>,
    //   preimages: Vec<Preimage>,
    //   guarantees: Vec<Guarantee>,
    //   assurances: Vec<Assurance>,
    //   disputes: Vec<Dispute>,
}

impl Block {
    pub async fn get(pool: &PgPool, slot: i32) -> Result<Self> {
        let raw: String = query_scalar!("SELECT raw FROM blocks WHERE slot = $1", slot)
            .fetch_one(pool)
            .await?;

        // TODO header
        // TODO extrinsic

        Ok(Self { slot, raw })
    }

    pub async fn insert(pool: &PgPool, block: &JamBlock) -> Result<()> {
        let raw = serde_json::to_string(&block.clone().to_json()).unwrap_or("".to_owned());
        let slot = block.header.slot as i32;

        Header::insert(pool, slot, &block.header).await?;

        // save epoch
        if let Some(epoch) = &block.header.epoch_mark {
            Epoch::insert(pool, slot, epoch).await?;
        }

        // save tickets
        if let Some(tickets) = &block.header.tickets_mark {
            for ticket in tickets {
                Ticket::insert(pool, slot, ticket).await?;
            }
        }

        // FIXME if this ticket envelop is related to header's tickets
        for envelope in block.extrinsic.tickets.iter() {
            Envelope::insert(pool, slot, envelope).await?;
        }

        // save preimages
        for preimage in block.extrinsic.preimages.iter() {
            Preimage::insert(pool, slot, preimage).await?;
        }

        // save guarantee
        for guarantee in block.extrinsic.guarantees.iter() {
            Guarantee::insert(pool, slot, guarantee).await?;
        }

        // save assurance
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

        query!("INSERT INTO blocks (slot,raw) VALUES ($1,$2)", slot, raw)
            .execute(pool)
            .await?;

        Ok(())
    }
}
