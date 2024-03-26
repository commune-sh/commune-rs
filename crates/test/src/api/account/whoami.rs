use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

pub async fn handler(TypedHeader(access_token): TypedHeader<Authorization<Bearer>>) -> Response {
    use commune::account::whoami::service;

    match service(access_token.token()).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to associate access token with user");

            e.into_response()
        }
    }
}
