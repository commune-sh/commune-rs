use axum::{
    extract::Path,
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse, Response},
};
use axum_extra::extract::cookie::Cookie;
use base64::{
    alphabet,
    engine::{general_purpose, GeneralPurpose},
    Engine,
};
use email_address::EmailAddress;
use ring::digest;
use time::Duration;

pub async fn handler(Path(address): Path<EmailAddress>) -> Response {
    use commune::account::email::service;

    match service(address).await {
        Ok(token) => {
            let engine = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

            let token_sha256 = digest::digest(&digest::SHA256, token.as_bytes());
            let token_sha256_b64 = engine.encode(token_sha256);

            let cookie = Cookie::build(("registration-token", token_sha256_b64))
                .max_age(Duration::minutes(60));

            ((), AppendHeaders([(SET_COOKIE, cookie.to_string())])).into_response()
        }
        Err(e) => {
            tracing::warn!(?e, "failed to send verification email");

            e.into_response()
        }
    }
}
