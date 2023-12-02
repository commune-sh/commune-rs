pub mod create;

use axum::routing::post;
use axum::Router;

use crate::services::SharedServices;

pub struct Account;

impl Account {
    pub fn routes() -> Router<SharedServices> {
        Router::new().route("/", post(create::handler))
    }
}
