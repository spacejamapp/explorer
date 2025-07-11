//! Graphql related stuffs

pub use cors::Cors;
use std::net::SocketAddr;

mod cors;
mod service;

/// Config for graphql
#[derive(Debug, Clone)]
pub struct Graphql {
    /// The graphql server address
    pub endpoint: SocketAddr,

    /// CORS configuration
    pub cors: Cors,
}
