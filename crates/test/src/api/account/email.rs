use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use email_address::EmailAddress;

pub async fn handler(Path(email): Path<EmailAddress>) -> Response {
    use commune::account::email::service;

    match service(email).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            tracing::warn!(?e, "failed to handle email verification");

            e.into_response()
        }
    }
}
