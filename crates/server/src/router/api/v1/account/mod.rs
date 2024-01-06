pub mod email;
pub mod login;
pub mod root;
pub mod verify_code;
pub mod verify_code_email;

use axum::routing::{get, post};
use axum::Router;

use crate::services::SharedServices;

pub struct Account;

impl Account {
    pub fn routes() -> Router<SharedServices> {
        let verify = Router::new()
            .route("/code", post(verify_code::handler))
            .route("/code/email", post(verify_code_email::handler));

        Router::new()
            .route("/", post(root::handler))
            .route("/email/:email", get(email::handler))
            .route("/login", post(login::handler))
            .nest("/verify", verify)
    }
}
