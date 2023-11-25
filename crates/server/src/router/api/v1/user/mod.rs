pub mod register;

use axum::routing::post;
use axum::Router;

use crate::services::SharedServices;

pub struct User;

impl User {
    pub fn routes() -> Router<SharedServices> {
        Router::new().route("/register", post(register::handler))
    }
}
