use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;

use crate::router::api::ApiError;
use crate::router::middleware::AccessToken;
use crate::services::SharedServices;

#[derive(Deserialize)]
pub struct CreateVoteEventDto {
    relates_to: String,
    board_id: String,
}

// TODO: create a function that allows us to compose
// handlers to avoid copy-pasting with little changes
pub async fn up(
    Extension(services): Extension<SharedServices>,
    Extension(access_token): Extension<AccessToken>,
    Json(payload): Json<CreateVoteEventDto>,
) -> Response {
        let request = services.commune.events.send_reaction(
            payload.board_id,
            payload.relates_to,
            "up".into(),
            access_token.into(),
        ).await;

    match request {
        Ok(resp) => {
            let mut response = Json(resp).into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => ApiError::from(err).into_response(),
    }
}

pub async fn down(
    Extension(services): Extension<SharedServices>,
    Extension(access_token): Extension<AccessToken>,
    Json(payload): Json<CreateVoteEventDto>,
) -> Response {
        let request = services.commune.events.send_reaction(
            payload.board_id,
            payload.relates_to,
            "down".into(),
            access_token.into(),
        ).await;

    match request {
        Ok(resp) => {
            let mut response = Json(resp).into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => ApiError::from(err).into_response(),
    }
}
