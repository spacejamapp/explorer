//! The models for jamscan
//!
//! TODO: need to re-implement the core types of spacejam here for implementing
//! the sugar interfaces of sqlx.

mod epoch;
mod header;
mod ticket;

pub use epoch::Epoch;
pub use header::Header;
pub use ticket::Ticket;
