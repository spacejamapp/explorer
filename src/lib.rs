//! The backend of Jamscan

#[macro_use]
extern crate sqlx;

pub use manager::Manager;

pub mod graphql;
mod manager;
mod models;
pub mod schema;
