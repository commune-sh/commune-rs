use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use matrix::ruma_common::OwnedMxcUri;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub avatar_url: OwnedMxcUri,
}

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Payload>,
) -> Response {
    use commune::profile::avatar_url::service;

    match service(access_token.token(), payload.avatar_url).await {
        Ok(_) => Json(crate::EmptyBody {}).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to update avatar");

            e.into_response()
        }
    }
}
