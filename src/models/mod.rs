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
mod ticket;
mod validator;

pub use assurance::Assurance;
pub use block::Block;
pub use core::Core;
pub use dispute::{DisputeCulprit, DisputeFault, DisputeVerdict};
pub use envelope::Envelope;
pub use epoch::Epoch;
pub use guarantee::Guarantee;
pub use header::Header;
pub use preimage::Preimage;
pub use ticket::Ticket;
pub use validator::Validator;
