pub mod legacy;

use axum::Router;

pub struct Events;

impl Events {
    pub fn routes() -> Router {
        Router::new().merge(legacy::Legacy::routes())
    }
}
