use rand::Rng;

use crate::env::Env;

pub async fn login() -> Result<matrix::client::session::login::Response, reqwest::Error> {
    let client = Env::new().await;

    let register_resp = crate::register::register(&client).await.unwrap();

    dbg!(&register_resp);

    let resp = client
        .post("/_commune/client/r0/login")
        .json(&router::api::session::login::Payload::new(
            register_resp.user_id.as_str(),
            "verysecure".into(),
        ))
        .send()
        .await
        .unwrap();

    dbg!(&resp);

    resp.json::<matrix::client::session::login::Response>()
        .await
}

#[tokio::test]
async fn login_test() {
    let resp = login().await.unwrap();

    dbg!(&resp);
    // assert!(!resp.access_token.is_empty());
}
