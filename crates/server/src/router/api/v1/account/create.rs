use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use commune::user::model::User;
use commune::user::service::CreateAccountDto;

use crate::router::api::ApiError;
use crate::services::SharedServices;

#[instrument(skip(services, payload))]
pub async fn handler(
    State(services): State<SharedServices>,
    Json(payload): Json<UserRegisterPayload>,
) -> Response {
    let dto = CreateAccountDto::from(payload);

    match services.commune.user.register(dto).await {
        Ok(user) => {
            let mut response = Json(UserRegisterResponse::from(user)).into_response();

            *response.status_mut() = StatusCode::CREATED;
            response
        }
        Err(err) => {
            tracing::warn!(?err, "Failed to register user");
            ApiError::from(err).into_response()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserRegisterPayload {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl From<UserRegisterPayload> for CreateAccountDto {
    fn from(payload: UserRegisterPayload) -> Self {
        Self {
            username: payload.username,
            password: payload.password.into(),
            email: payload.email,
            // FIXME: These should be queried from somewhere
            session: "test".to_string(),
            code: "test".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserRegisterResponse {
    pub username: String,
}

impl From<User> for UserRegisterResponse {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
        }
    }
}
