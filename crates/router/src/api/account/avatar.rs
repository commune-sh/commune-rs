use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use matrix::ruma_common::OwnedMxcUri;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub mxc_uri: OwnedMxcUri,
}

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Payload>,
) -> Response {
    use commune::profile::avatar::update::service;

    match service(access_token.token(), payload.mxc_uri).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to update avatar");

            e.into_response()
        }
    }
}
