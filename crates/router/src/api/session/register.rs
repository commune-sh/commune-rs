use axum::{
    response::{IntoResponse, Response},
    Json,
};
use commune::util::secret::Secret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    username: String,
    password: Secret,
}

impl Payload {
    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password: Secret::new(password.into()),
        }
    }
}

pub async fn handler(Json(payload): Json<Payload>) -> Response {
    use commune::session::register::service;

    match service(
        &commune::commune().handle,
        &payload.username,
        &payload.password,
    )
    .await
    {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to register user");

            e.into_response()
        }
    }
}
