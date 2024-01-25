pub mod email;
pub mod login;
pub mod root;
pub mod session;
pub mod verify_code;
pub mod verify_code_email;

use axum::routing::{get, post};
use axum::{middleware, Router};

use crate::router::middleware::auth;

pub struct Account;

impl Account {
    pub fn routes() -> Router {
        Router::new()
            .route("/session", get(session::handler))
            .route_layer(middleware::from_fn(auth))
            .route("/", post(root::handler))
            .route("/login", post(login::handler))
            .route("/email/:email", get(email::handler))
            .nest(
                "/verify",
                Router::new()
                    .route("/code", post(verify_code::handler))
                    .route("/code/email", post(verify_code_email::handler)),
            )
    }
}
