use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub display_name: String,
}

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Payload>,
) -> Response {
    use commune::profile::avatar::update::service;

    match service(access_token.token(), payload.display_name).await {
        Ok(_) => ().into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to update display name");

            e.into_response()
        }
    }
}
