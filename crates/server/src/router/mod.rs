pub mod api;
pub mod middleware;

use axum::{Extension, Router};

use crate::services::SharedServices;

pub fn make_router(service: SharedServices) -> Router {
    Router::new()
        .merge(api::Api::routes())
        .layer(Extension(service))
}
