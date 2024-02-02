use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use commune::events::{AnyMessageLikeEvent, Raw};
use serde::Deserialize;

use crate::router::api::ApiError;
use crate::router::middleware::AccessToken;
use crate::services::SharedServices;

#[derive(Deserialize)]
pub struct CreateBoardEventDto {
    content: Raw<AnyMessageLikeEvent>,
    board_id: String,
    is_reply: bool,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Extension(access_token): Extension<AccessToken>,
    Json(payload): Json<CreateBoardEventDto>,
) -> Response {
    let request = match payload.is_reply {
        true => services.commune.events.send_reply(
            payload.content.cast(),
            payload.board_id,
            access_token.into(),
        ).await,
        false => services.commune.events.send_post(
            payload.content.cast(),
            payload.board_id,
            access_token.into(),
        ).await,
    };

    match request {
        Ok(resp) => {
            let mut response = Json(resp).into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => ApiError::from(err).into_response(),
    }
}
