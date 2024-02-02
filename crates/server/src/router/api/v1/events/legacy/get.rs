use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

use crate::router::api::ApiError;
use crate::router::middleware::AccessToken;
use crate::services::SharedServices;

pub struct GetBoardEventDto {
    pub board_id: String,
    pub event_id: String,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Extension(access_token): Extension<AccessToken>,
    Extension(payload): Extension<GetBoardEventDto>,
) -> Response {
    match services
        .commune
        .events
        .get_post(payload.board_id, payload.event_id, access_token.into(), Some(10)).await
    {
        Ok(resp) => {
            let mut response = Json(resp).into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => ApiError::from(err).into_response(),
    }
}
