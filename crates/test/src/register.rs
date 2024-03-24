use rand::Rng;

use crate::env::Env;

pub async fn register(
    client: &Env,
) -> Result<matrix::client::session::register::Response, reqwest::Error> {
    let resp = client
        .post("/_commune/client/r0/register")
        .json(&router::api::session::register::Payload::new(
            format!("steve-{}", rand::thread_rng().gen::<u8>()),
            "verysecure".into(),
        ))
        .send()
        .await
        .unwrap();

    tracing::info!(?resp);

    resp.json::<matrix::client::session::register::Response>()
        .await
}

// #[tokio::test]
async fn register_test() {
    let client = Env::new().await;

    let resp = register(&client).await.unwrap();

    assert!(!resp.access_token.is_empty());
}
