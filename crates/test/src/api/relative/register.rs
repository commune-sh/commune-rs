use commune::util::secret::Secret;
use rand::seq::IteratorRandom;

use matrix::client::register::root::*;
use router::api::relative::register;

use crate::env::Env;

pub async fn register(client: &Env) -> Result<Response, reqwest::Error> {
    let allowed = ('0'..='9')
        .chain('a'..='z')
        .chain(['-', '.', '=', '_', '/', '+']);
    let username = allowed
        .choose_multiple(&mut rand::thread_rng(), 8)
        .into_iter()
        .collect();

    tracing::info!(?username);

    let resp = client
        .post("/_commune/client/r0/register")
        .json(&register::Payload {
            username,
            password: Secret::new("verysecure"),
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
