//! Crate to centralize all Matrix dependencies.
//!
//! Reexports `matrix_sdk` and provides implementations on Matrix Admin API.

/// Implementation on the Administrator API of Matrix
///
/// Refer: https://matrix-org.github.io/synapse/latest/usage/administration/index.html
pub mod admin;

/// The official Matrix Rust SDK.
///
/// # Project State
///
/// As of today this SDK is still in beta and is not yet ready for production,
/// so we make use of the repo at a specific commit.
///
/// Refer: https://github.com/matrix-org/matrix-rust-sdk
pub use matrix_sdk as sdk;
