//! The models for jamscan
//!
//! TODO: need to re-implement the core types of spacejam here for implementing
//! the sugar interfaces of sqlx.

mod epoch;
mod extrinsic;
mod header;

pub use epoch::Epoch;
pub use extrinsic::ticket::Ticket;
pub use header::Header;
