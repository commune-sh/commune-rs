//! This module is the root of the client-server API.
//!
//! reference: https://spec.matrix.org/unstable/client-server-api

pub mod account;
pub mod login;
pub mod logout;
pub mod profile;
pub mod register;

pub mod create_room;
pub mod directory;
pub mod membership;
pub mod rooms;

pub mod uiaa;
