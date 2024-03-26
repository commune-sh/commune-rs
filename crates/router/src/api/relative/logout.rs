use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};

#[axum::debug_handler]
pub async fn handler(TypedHeader(access_token): TypedHeader<Authorization<Bearer>>) -> Response {
    use commune::account::logout::service;

    match service(access_token.token()).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to logout user");

            e.into_response()
        }
    }
}
