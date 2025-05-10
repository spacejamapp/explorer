//! SpaceJam runtime hook for jamscan

use anyhow::Result;
use score::{
    Block as JamBlock, EPOCH_LENGTH, OpaqueHash, ServiceId, TimeSlot, state::key,
    statistic::Statistics,
};
use sqlx::PgPool;
use std::collections::{BTreeMap, HashMap};

use crate::models::{Block, Epoch};

/// Spacejam runtime hook for jamscan
pub struct JamScanHook(PgPool);

impl JamScanHook {
    /// Create a new JamScanHook
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

impl runtime::Hook for JamScanHook {
    async fn on_finalized_block(&self, block: JamBlock) -> Result<()> {
        println!("block: {}", block.header.slot);
        // save the block
        Block::insert(&self.0, &block).await?;

        Ok(())
    }

    fn on_diff(
        &self,
        hash: OpaqueHash,
        diff: HashMap<OpaqueHash, Vec<u8>>,
    ) -> impl Future<Output = Result<()>> {
        async move {
            let mut epoch = 0;
            let mut statistics = Statistics::default();

            let mut data = BTreeMap::new();
            let mut preimage = BTreeMap::new();
            let mut request = BTreeMap::new();
            let mut svalue = BTreeMap::new();

            for (key, value) in diff {
                // get current timeslot
                if key == key::TIMESLOT {
                    let mut bytes = [0u8; 4];
                    bytes.copy_from_slice(&value);
                    let slot = TimeSlot::from_le_bytes(bytes);
                    epoch = slot / EPOCH_LENGTH + 1;
                    continue;
                }

                // get current statistics data
                if key == key::STATISTICS {
                    statistics = jamcodec::decode(&value)?;
                    continue;
                }

                // skip the key that is not related to service
                if key[1..].iter().all(|b| *b == 0) {
                    continue;
                }

                // call the hook
                self.on_key_value(hash, key, &value)?;

                // service info storage
                if key[8..].iter().all(|b| *b == 0) {
                    let mut service = [0u8; 4];
                    service[0] = key[1];
                    service[1] = key[3];
                    service[2] = key[5];
                    service[3] = key[7];
                    data.insert(ServiceId::from_le_bytes(service), value);
                    continue;
                }

                // get the service id
                let service = {
                    let mut sbuf = [0u8; 4];
                    sbuf[0] = key[0];
                    sbuf[1] = key[2];
                    sbuf[2] = key[4];
                    sbuf[3] = key[6];

                    ServiceId::from_le_bytes(sbuf)
                };

                let prefix = {
                    let mut pbuf = [0u8; 4];
                    pbuf[0] = key[1];
                    pbuf[1] = key[3];
                    pbuf[2] = key[5];
                    pbuf[3] = key[7];
                    pbuf
                };

                match prefix {
                    key::ACCOUNT_STORAGE_PREFIX => {
                        svalue.insert(service, (key.to_vec(), value));
                    }
                    key::ACCOUNT_PREIMAGE_PREFIX => {
                        preimage.insert(service, (key.to_vec(), value));
                    }
                    length => {
                        let length = u32::from_le_bytes(length);
                        request.insert(service, (length, key.to_vec(), value));
                    }
                }
            }

            println!("epoch: {}", epoch);
            println!("statistics: {}", statistics.vals_current.len());

            let mut blocks = 0;
            let mut tickets = 0;
            let mut preimages = 0;
            let mut preimages_size = 0;
            let mut guarantees = 0;
            let mut assurances = 0;
            for record in statistics.vals_current {
                blocks += record.blocks;
                tickets += record.tickets;
                preimages += record.pre_images;
                preimages_size += record.pre_images_size;
                guarantees += record.guarantees;
                assurances += record.assurances;
            }
            let _ = Epoch::statistic(
                &self.0,
                epoch as i32,
                blocks as i32,
                tickets as i32,
                preimages as i32,
                preimages_size as i32,
                guarantees as i32,
                assurances as i32,
            )
            .await;

            // handle service data

            // handle service value

            // handle service preimage

            // handle service request

            Ok(())
        }
    }
}
