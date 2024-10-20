use axum::Router;

pub(crate) mod api;
pub(crate) mod error;
pub(crate) mod router;

pub(crate) use error::Error;

#[tokio::main]
pub(crate) async fn main() {
}
