pub mod create;
pub mod login;
pub mod verify;

use axum::routing::post;
use axum::Router;

use crate::services::SharedServices;

pub struct Account;

impl Account {
    pub fn routes() -> Router<SharedServices> {
        Router::new()
            .route("/", post(create::handler))
            .route("/login", post(login::handler))
            .route("/verify/code", post(verify::code::handler))
    }
}
