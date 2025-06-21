//! The backend of Jamscan

#[macro_use]
extern crate sqlx;

pub use manager::Manager;

mod manager;
mod models;
pub mod schema;
