pub mod api;
pub mod middleware;

use axum::Router;

use crate::services::SharedServices;

pub fn make_router() -> Router<SharedServices> {
    Router::new().nest("/api", api::Api::routes())
}
