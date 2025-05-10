//! The backend of Jamscan

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate tracing;

pub use hook::JamScanHook;

mod hook;
mod models;
pub mod schema;
