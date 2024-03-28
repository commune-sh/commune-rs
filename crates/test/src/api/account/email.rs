use commune::util::secret::Secret;
use matrix::client::register::root::*;
use router::api::register::root as register;
use serde::Deserialize;
use url::form_urlencoded::byte_serialize;

use crate::{env::Env, util::generate_comforming_localpart};

pub async fn verify_email(client: &Env) -> Result<Response, reqwest::Error> {
    let resp = client
        .get(
            format!(
                "/_commune/client/r0/register/email/{}",
                byte_serialize("commune@example.org".as_bytes()).collect::<String>()
            )
            .as_str(),
        )
        .send()
        .await?;

    tracing::info!(resp = ?resp.headers());

    let cookie = resp.headers().get("set-cookie").unwrap();

    // TODO: make this type-safe
    let s = cookie.to_str().unwrap();
    let query = s.split(';').next().unwrap();
    let token_hash = query.split('=').nth(1).unwrap();

    tracing::info!(token_hash);

    let resp = reqwest::get("http://localhost:1080/api/messages").await?;
    let emails: Vec<EmailId> = resp.json().await?;

    let EmailId { id } = emails.last().unwrap();
    let resp = reqwest::get(format!("http://localhost:1080/api/message/{id}")).await?;

    let EmailBody { text } = resp.json().await?;
    let code: String = text.chars().filter(|c| c.is_digit(10)).collect();

    let engine = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let token_sha256 = digest::digest(&digest::SHA256, &token.as_bytes());
    let token_sha256_b64 = engine.encode(token_sha256);

    let username = generate_comforming_localpart();

    tracing::info!(?username);

    let resp = client
        .post("/_commune/client/r0/register")
        .json(&register::Payload {
            username,
            password: Secret::new("verysecure"),
            registration_token: Some(code),
        })
        .send()
        .await?;

    resp.json().await
}

#[derive(Deserialize)]
struct EmailId {
    pub id: String,
}

#[derive(Deserialize)]
struct EmailBody {
    pub text: String,
}

#[tokio::test]
async fn verify_email_test() {
    let client = Env::new().await;

    let resp = verify_email(&client).await.unwrap();

    tracing::info!(?resp);
}
