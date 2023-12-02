pub mod account;

use axum::Router;

use crate::services::SharedServices;

pub struct V1;

impl V1 {
    pub fn routes() -> Router<SharedServices> {
        Router::new().nest("/account", account::Account::routes())
    }
}
