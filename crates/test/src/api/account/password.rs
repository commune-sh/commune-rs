use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use commune::util::secret::Secret;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    username: String,
    password: Secret,
    new_password: Secret,
}

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Payload>,
) -> Response {
    use commune::account::password::service;

    match service(
        access_token.token(),
        payload.username,
        payload.password,
        payload.new_password,
    )
    .await
    {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to reset password");

            e.into_response()
        }
    }
}
