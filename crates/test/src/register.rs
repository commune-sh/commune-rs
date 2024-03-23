use commune::util::secret::Secret;
use rand::Rng;

use crate::env::Env;

#[tokio::test]
async fn register() {
    let client = Env::new().await;

    let resp = client
        .post("/_commune/client/r0/register")
        .json(&router::api::session::register::Payload {
            username: format!("steve-{}", rand::thread_rng().gen::<u8>()),
            password: Secret::new("verysecure"),
        })
        .send()
        .await
        .unwrap();

    tracing::info!(?resp);

    let resp = resp
        .json::<matrix::client::session::register::Response>()
        .await
        .unwrap();

    assert!(!resp.access_token.is_empty());
}
