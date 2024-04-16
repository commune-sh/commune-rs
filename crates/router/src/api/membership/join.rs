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

pub async fn handler(
    TypedHeader(access_token): TypedHeader<Authorization<Bearer>>,
    Path(space_id): Path<OpaqueId>,
) -> Response {
    use commune::membership::join::service;

    match service(access_token.token(), space_id, None).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to join space");

            e.into_response()
        }
    }
}
