//! The spacejam cache in memory

use crate::models::{
    Assurance, Block, DisputeCulprit, DisputeFault, DisputeVerdict, Guarantee, Preimage, Service,
    Ticket,
};
use anyhow::Result;
use async_graphql::{ComplexObject, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// Spacejam global state
#[derive(SimpleObject, Deserialize, Serialize, Clone)]
#[graphql(complex)]
pub struct Spacejam {
    /// The total number of tickets
    pub tickets: i64,

    /// The total number of preimages
    pub preimages: i64,

    /// The total number of guarantees
    pub guarantees: i64,

    /// The total number of assurances
    pub assurances: i64,

    /// The total number of disputes verdicts
    pub disputes_verdicts: i64,

    /// The total number of disputes culprits
    pub disputes_culprits: i64,

    /// The total number of disputes faults
    pub disputes_faults: i64,

    /// The total number of blocks
    pub blocks: i64,

    /// The slot of the last finalized block
    pub finalized: u32,

    /// The total number of services
    pub services: i64,
}

#[ComplexObject]
impl Spacejam {
    /// The total number of extrinsics
    pub async fn extrinsics(&self) -> i64 {
        self.tickets
            + self.preimages
            + self.guarantees
            + self.assurances
            + self.disputes_verdicts
            + self.disputes_culprits
            + self.disputes_faults
    }
}

impl Spacejam {
    /// Initialize the spacejam cache
    pub async fn init(pg: &PgPool) -> Result<Self> {
        let tickets = Ticket::count(pg).await?;
        let preimages = Preimage::count(pg).await?;
        let guarantees = Guarantee::count(pg).await?;
        let assurances = Assurance::count(pg).await?;
        let disputes_verdicts = DisputeVerdict::count(pg).await?;
        let disputes_culprits = DisputeCulprit::count(pg).await?;
        let disputes_faults = DisputeFault::count(pg).await?;
        let blocks = Block::count(pg).await?;
        let services = Service::count(pg).await?;

        Ok(Spacejam {
            tickets,
            preimages,
            guarantees,
            assurances,
            disputes_verdicts,
            disputes_culprits,
            disputes_faults,
            blocks,
            services,
            finalized: 0,
        })
    }
}
