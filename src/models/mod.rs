//! The models for jamscan
//!
//! TODO: need to re-implement the core types of spacejam here for implementing
//! the sugar interfaces of sqlx.

mod assurance;
mod block;
mod core;
mod dispute;
mod envelope;
mod epoch;
mod guarantee;
mod header;
mod preimage;
mod service;
mod ticket;
mod validator;
mod work_result;

pub use assurance::Assurance;
pub use block::Block;
pub use core::Core;
pub use dispute::{DisputeCulprit, DisputeFault, DisputeVerdict};
pub use envelope::Envelope;
pub use epoch::Epoch;
pub use guarantee::Guarantee;
pub use header::Header;
pub use preimage::Preimage;
pub use service::Service;
pub use ticket::Ticket;
pub use validator::Validator;
pub use work_result::WorkResult;

#[inline]
pub fn hex<T: AsRef<[u8]>>(bytes: T) -> String {
    "0x".to_owned() + &hex::encode(bytes)
}
