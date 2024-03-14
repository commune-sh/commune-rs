use axum::{
    response::{IntoResponse, Response},
    Json,
};

pub async fn handler() -> Response {
    use commune::session::logout::service;

    match service(&commune::commune().handle).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to logout user");

            e.into_response()
        }
    }
}
