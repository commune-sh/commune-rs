use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use matrix::ruma_common::OwnedRoomOrAliasId;

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Path(room_or_alias_id): Path<OwnedRoomOrAliasId>,
) -> Response {
    use commune::membership::join::service;

    match service(access_token.token(), room_or_alias_id, None).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to join space");

            e.into_response()
        }
    }
}
