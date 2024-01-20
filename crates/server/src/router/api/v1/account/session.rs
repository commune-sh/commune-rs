use axum::extract::State;

use axum::response::{IntoResponse, Response};
use axum::Extension;
use commune::account::model::Account;

use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::services::SharedServices;

use super::root::{AccountMatrixCredentials, AccountSpace};

#[instrument(skip(_services))]
pub async fn handler(
    Extension(whoami): Extension<Account>,
    State(_services): State<SharedServices>,
) -> Response {
    println!("whoami: {:?}", whoami);
    "Hello World!".into_response()
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSessionResponse {
    pub credentials: AccountMatrixCredentials,
    pub rooms: Vec<String>,
    pub spaces: Vec<AccountSpace>,
}
