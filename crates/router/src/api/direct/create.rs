use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use matrix::ruma_common::OwnedUserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub name: Option<String>,
    pub topic: Option<String>,
    pub invite: Vec<OwnedUserId>,
}

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Payload>,
) -> Response {
    use commune::direct::create::service;

    match service(
        access_token.token(),
        payload.name,
        payload.topic,
        payload.invite
    )
    .await
    {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to create room");

            e.into_response()
        }
    }
}
