//! SpaceJam runtime hook for jamscan

use sqlx::Executor;
use sqlx::PgPool;

/// Spacejam runtime hook for jamscan
#[derive(Clone)]
pub struct JamScanHook(PgPool);

impl JamScanHook {
    /// Create a new JamScanHook
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

impl From<PgPool> for JamScanHook {
    fn from(pool: PgPool) -> Self {
        Self(pool)
    }
}

impl runtime::Hook for JamScanHook {
    async fn on_finalized_block(&self, block: score::Block) -> anyhow::Result<()> {
        // save the header
        let block_hash = hex::encode(block.header.hash()?);
        let parent = hex::encode(block.header.parent);
        let parent_state_root = hex::encode(block.header.parent_state_root);
        let extrinsic_hash = hex::encode(block.header.extrinsic_hash);
        let slot = block.header.slot;
        let epoch_mark = if let Some(epoch) = block.header.epoch_mark {
            let entropy = hex::encode(epoch.entropy);
            let tickets_entropy = hex::encode(epoch.tickets_entropy);
            // save validator, and use ed25519 as the primary key
            let mut validators = vec![];
            let mut validators_bandersnatches = vec![];
            for validator in epoch.validators {
                validators.push(hex::encode(validator.ed25519));
                validators_bandersnatches.push(hex::encode(validator.bandersnatch));
            }

            // insert epoch
            self.0.execute(sqlx::query!(
                "INSERT INTO epoches (entropy,tickets_entropy,validators,validators_bandersnatches) VALUES ($1,$2,$3,$4)",
                entropy,
                tickets_entropy,
                &validators,
                &validators_bandersnatches
            )).await?;

            Some(entropy)
        } else {
            None
        };
        let tickets_mark = if let Some(tickets) = block.header.tickets_mark {
            // save ticket
            let mut saved_tickets = vec![];
            for ticket in tickets {
                let ticket_id = hex::encode(ticket.id);
                self.0
                    .execute(sqlx::query!(
                        "INSERT INTO tickets (ticket_id,attempt) VALUES ($1, $2)",
                        ticket_id,
                        ticket.attempt as i16,
                    ))
                    .await?;

                saved_tickets.push(ticket_id);
            }

            saved_tickets
        } else {
            vec![]
        };
        let offenders_mark = block
            .header
            .offenders_mark
            .iter()
            .map(|v| hex::encode(v))
            .collect::<Vec<String>>();
        let author_index = block.header.author_index;
        let entroy_source = hex::encode(block.header.entropy_source);
        let seal = hex::encode(block.header.seal);

        self.0
            .execute(sqlx::query!(
                "INSERT INTO headers (hash,parent,parent_state_root,extrinsic_hash,slot,epoch_mark,tickets_mark,offenders_mark,author_index,entropy_source,seal) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
                block_hash,
                parent,
                parent_state_root,
                extrinsic_hash,
                slot as i32,
                epoch_mark,
                &tickets_mark,
                &offenders_mark,
                author_index as i32,
                entroy_source,
                seal,
            ))
            .await?;

        // save the body(extrinsic)

        Ok(())
    }
}
