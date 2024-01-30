//! Crate to centralize all Matrix dependencies.
//!
//! Reexports `matrix_sdk` and provides implementations on Matrix Admin API.

mod http;

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

/// Implementation of our custom space events.
mod space;

/// Here we intentionally shadow `ruma::events::space` in favor of our own.
/// `ruma::events` extended with the implementation of our custom space events.
pub mod events {
    pub use ruma_events::*;

    pub mod space {
        pub use ruma_events::space::*;

        pub use crate::space::*;
    }
}
