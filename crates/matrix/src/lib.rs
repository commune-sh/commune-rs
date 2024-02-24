//! Crate to centralize all Matrix dependencies.
//!
//! Reexports `matrix_sdk` and provides implementations on Matrix Admin API.

// mod http;

// pub mod filter;

// pub use http::Client;

/// Ruma re-exports
// pub use ruma_common;
// pub use ruma_events;

// mod api {
mod admin;
    // mod client;
// }

use ruma_common::api::error::MatrixError as Error;
