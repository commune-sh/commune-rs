use axum::{
    response::{IntoResponse, Response},
    Json,
};
use commune::util::secret::Secret;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    username: String,
    password: Secret,
}

pub async fn handler(
    Json(payload): Json<Payload>,
) -> Response {
    use commune::session::login::service;

    match service(&commune::commune().handle, &payload.username, &payload.password).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to login user");

            e.into_response()
        }
    }
}
