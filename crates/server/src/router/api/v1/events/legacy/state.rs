use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use commune::events::space::state::SpaceRestrictionEventContent;
use commune::events::{Raw, StateEventContent};
use serde::Deserialize;

use crate::router::api::ApiError;
use crate::router::middleware::AccessToken;
use crate::services::SharedServices;

#[derive(Deserialize)]
pub struct CreateStateDto<C: StateEventContent> {
    content: Raw<C>,
    board_id: String,
    state_key: String,
}

/// temporary workaround: our handler should take the event type as a path parameter i.e.
/// /_matrix/client/v3/rooms/{roomId}/state/{eventType}/{stateKey}
pub type CreateRestrictionDto = CreateStateDto<SpaceRestrictionEventContent>;

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Extension(access_token): Extension<AccessToken>,
    Json(payload): Json<CreateRestrictionDto>,
) -> Response {
    match services
        .commune
        .events
        .send_state(
            payload.content,
            payload.board_id,
            payload.state_key,
            access_token.into(),
        )
        .await
    {
        Ok(resp) => {
            let mut response = Json(resp).into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => ApiError::from(err).into_response(),
    }
}
