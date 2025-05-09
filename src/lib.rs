//! The backend of Jamscan

#[macro_use]
extern crate sqlx;

pub use hook::JamScanHook;

mod hook;
mod models;
pub mod schema;
