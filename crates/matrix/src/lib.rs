//! Crate to centralize all Matrix dependencies.
//!
//! Reexports `matrix_sdk` and provides implementations on Matrix Admin API.

mod http;

mod error;

pub mod filter;

pub use http::Client;

/// Implementation on the Administrator API of Matrix
///
/// Refer: https://matrix-org.github.io/synapse/latest/usage/administration/index.html
pub mod admin;

/// Implementation on the Client API of Matrix
///
/// Different to the Matrix SDK, no user state is kept in the Client instance,
/// this is equivalent to making cURL requests to the Matrix server.
pub mod client;

/// Ruma re-exports
pub use ruma_common;
pub use ruma_events;
