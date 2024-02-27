//! This library deals with forwarding Matrix requests to the server.
//! Comments have been used sparingly as the specification contains all the technical details.

//! We rely on `ruma` to abstract away the boilerplate introduced by HTTP requests,
//! without sacrificing flexibility by defining our own request and response types.
//!
//! reference: https://docs.ruma.io/ruma_common/api/index.html

pub mod admin;
pub mod client;

#[allow(unused_imports)]
use ruma_common::api::error::MatrixError as Error;
