use std::time::Duration;

use commune::util::secret::Secret;

#[tokio::test]
async fn register() {
    tracing_subscriber::fmt().init();

    commune::init();

    let public_loopback = commune::commune().config.public_loopback;
    let port = commune::commune().config.port;

    tokio::spawn(async move {
        router::serve(public_loopback, port.unwrap())
            .await
            .expect("failed to start server");
    });

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let addr = url::Url::parse(&format!(
        "http://127.0.0.1:{}/_commune/client/r0/register",
        port.unwrap()
    ))
    .unwrap();

    let resp = client
        .post(addr)
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
