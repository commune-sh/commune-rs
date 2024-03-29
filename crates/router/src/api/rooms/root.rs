use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use matrix::ruma_common::OwnedRoomOrAliasId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub alias: Option<String>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub parent: OwnedRoomOrAliasId,
}

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Payload>,
) -> Response {
    use commune::rooms::create::service;

    match service(
        access_token.token(),
        payload.alias,
        payload.name,
        payload.topic,
    )
    .await
    {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to create space");

            e.into_response()
        }
    }
}
