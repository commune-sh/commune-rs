pub mod account;
pub mod events;

use axum::Router;

pub struct V1;

impl V1 {
    pub fn routes() -> Router {
        Router::new().nest("/account", account::Account::routes())
    }
}
