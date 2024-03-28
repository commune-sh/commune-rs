use axum::{
    response::{IntoResponse, Response},
    Json,
};
use commune::util::secret::Secret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub username: String,

    pub password: Secret,

    #[serde(default)]
    pub registration_token: Option<String>,
}

pub async fn handler(Json(payload): Json<Payload>) -> Response {
    use commune::account::register::service;

    match service(payload.username, payload.password, payload.registration_token).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to create account");

            e.into_response()
        }
    }
}
