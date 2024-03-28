use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};

pub async fn handler(Path(username): Path<String>) -> Response {
    use commune::account::username::service;

    match service(username).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to check username availability");

            e.into_response()
        }
    }
}
