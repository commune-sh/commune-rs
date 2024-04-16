use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use matrix::ruma_common::OwnedUserId;

pub async fn handler(Path(user_id): Path<OwnedUserId>) -> Response {
    use commune::profile::service;

    match service(&*user_id).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to retrieve profile of {user_id}");

            e.into_response()
        }
    }
}
