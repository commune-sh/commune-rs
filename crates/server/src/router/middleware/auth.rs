use axum::http::{header::AUTHORIZATION, Request};
use axum::middleware::Next;
use axum::response::Response;

use commune::util::secret::Secret;

use crate::router::api::ApiError;
use crate::services::SharedServices;

pub async fn auth<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, ApiError> {
    let access_token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(ApiError::unauthorized())?
        .to_owned();

    let services = request
        .extensions()
        .get::<SharedServices>()
        .ok_or_else(|| {
            tracing::error!("SharedServices not found in request extensions");
            ApiError::internal_server_error()
        })?;

    let user = services
        .commune
        .account
        .whoami(Secret::new(access_token))
        .await
        .map_err(|err| {
            tracing::error!("Failed to validate token: {}", err);
            ApiError::internal_server_error()
        })?;

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
