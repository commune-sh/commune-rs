use commune::util::secret::Secret;

use crate::env::Env;

#[tokio::test]
async fn register() {
    let client = Env::new().await;

    let resp = client
        .post("/_commune/client/r0/register")
        .json(&router::api::session::register::Payload {
            username: "steve".to_owned(),
            password: Secret::new("verysecure"),
        })
        .send()
        .await
        .unwrap();

    dbg!(resp.status());

    if resp.status().is_success() {
        let resp = resp
            .json::<matrix::client::session::register::Response>()
            .await
            .unwrap();

        assert!(!resp.access_token.is_empty());
    }
}
