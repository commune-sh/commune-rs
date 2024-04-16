use commune::util::secret::Secret;

use matrix::client::register::root::Response;
use router::api::register::root as register;

use crate::{env::Env, util::generate_comforming_localpart};

pub async fn register(client: &Env) -> Result<Response, reqwest::Error> {
    let username = generate_comforming_localpart();

    tracing::info!(?username);

    let resp = client
        .post("/_commune/client/r0/register")
        .json(&register::Payload {
            username,
            password: Secret::new("verysecure"),
            registration_token: None,
        })
        .send()
        .await?;

    resp.json().await
}

#[tokio::test]
async fn register_test() {
    let client = Env::new().await;

    let resp = register(&client).await.unwrap();

    tracing::info!(?resp);

    assert!(resp.access_token.is_some() && resp.access_token.map(|at| !at.is_empty()).unwrap());
}
