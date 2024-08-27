use axum::{
    response::{IntoResponse, Response},
    Json,
};
use commune::util::secret::Secret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub username: String,
    pub password: Secret,
}

pub async fn handler(Json(payload): Json<Payload>) -> Response {
    use commune::account::login::service;

    match service(&payload.username, &payload.password).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to login user");

            e.into_response()
        }
    }
}
