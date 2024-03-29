use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use commune::util::opaque_id::OpaqueId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub name: Option<String>,
    pub topic: Option<String>,
}

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Path(space_id): Path<OpaqueId>,
    Json(payload): Json<Payload>,
) -> Response {
    use commune::spaces::channels::create::service;

    match service(access_token.token(), space_id, payload.name, payload.topic).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to create channel for space");

            e.into_response()
        }
    }
}
