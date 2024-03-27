use commune::util::secret::Secret;
use matrix::client::login::*;
use router::api::relative::login;

use crate::{api::relative::register, env::Env};

pub async fn login(client: &Env) -> Result<Response, reqwest::Error> {
    let register_resp = register::register(&client).await.unwrap();

    tracing::info!(?register_resp);

    let resp = client
        .post("/_commune/client/r0/login")
        .json(&login::Payload {
            username: register_resp.user_id.into(),
            password: Secret::new("verysecure"),
        })
        .send()
        .await?;

    resp.json::<Response>().await
}

#[tokio::test]
async fn login_test() {
    let client = Env::new().await;

    let resp = login(&client).await.unwrap();

    tracing::info!(?resp);

    assert!(!resp.access_token.is_empty());
}
