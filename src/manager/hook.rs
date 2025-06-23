//! Spacejam runtime hook for jamscan

use crate::{
    Manager,
    models::{Block, Core, Epoch, Service},
};
use anyhow::Result;
use runtime::storage::Commit;
use score::{
    Block as JamBlock, EPOCH_LENGTH, OpaqueHash, ServiceId, TimeSlot, service::ServiceData,
    state::key, statistic::Statistics,
};
use std::collections::BTreeMap;

impl runtime::Hook for Manager {
    async fn on_finalized_block(&self, block: JamBlock) -> Result<()> {
        Block::insert(&self.pg, &block).await?;
        self.on_finalized_block(&block).await?;
        Ok(())
    }

    async fn on_diff(&self, hash: OpaqueHash, diff: Commit<[u8; 31], Vec<u8>>) -> Result<()> {
        let mut epoch = 0i32;
        let mut statistics = Statistics::default();
        let mut data = BTreeMap::new();
        let mut preimage = BTreeMap::new();
        let mut request = BTreeMap::new();
        let mut svalue = BTreeMap::new();

        for (key, value) in diff.iset() {
            // get current timeslot
            if *key == key::TIMESLOT {
                let mut bytes = [0u8; 4];
                bytes.copy_from_slice(value);
                let slot = TimeSlot::from_le_bytes(bytes);
                epoch = (slot / EPOCH_LENGTH + 1) as i32;
                tracing::debug!("epoch: {epoch}, slot: {slot}");
                continue;
            }

            // get current statistics data
            if *key == key::STATISTICS {
                statistics = jamcodec::decode(value)?;
                continue;
            }

            // skip the key that is not related to service
            if key[1..].iter().all(|b| *b == 0) {
                continue;
            }

            // call the hook
            self.on_key_value(hash, *key, value)?;

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

        // update epoch statistics
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
            &self.pg,
            epoch,
            blocks as i32,
            tickets as i32,
            preimages as i32,
            preimages_size as i32,
            guarantees as i32,
            assurances as i32,
        )
        .await;

        // update core statistics
        for (index, core) in statistics.cores.iter().enumerate() {
            let _ = Core::statistic(&self.pg, epoch, index as i32, core).await;
        }

        // handle service data
        for (sid, sdata) in data {
            if let Ok(service_data) = jamcodec::decode::<ServiceData>(sdata) {
                let _ = Service::insert(&self.pg, sid, &service_data).await;
            }
        }

        Ok(())
    }
}
