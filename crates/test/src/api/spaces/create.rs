use matrix::client::create_room::*;
use rand::{distributions::Uniform, prelude::Distribution};
use router::api::spaces::create::Payload;

use crate::{api::relative::login, env::Env};

pub async fn create_space(client: &Env) -> Result<Response, reqwest::Error> {
    let login_resp = login::login(&client).await.unwrap();

    tracing::info!(?login_resp);

    let uni = Uniform::new('0', '9');
    let id: String = uni.sample_iter(rand::thread_rng()).take(6).collect();

    let resp = client
        .post("/_commune/client/r0/spaces")
        .json(&Payload {
            alias: Some(format!("alias-{id}")),
            name: Some(format!("name-{id}")),
            topic: Some(format!("topic-{id}")),
        })
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", &login_resp.access_token),
        )
        .send()
        .await?;

    tracing::info!(?resp);

    resp.json().await
}

#[tokio::test]
async fn create_space_test() {
    let client = Env::new().await;

    let resp = create_space(&client).await.unwrap();

    tracing::info!(?resp);
}
