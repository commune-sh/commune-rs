use matrix::client::register::available::*;

use crate::{api::relative::register, env::Env, util::generate_comforming_localpart};

pub async fn available(client: &Env) -> Result<Response, reqwest::Error> {
    let register_resp = register::register(&client).await.unwrap();

    tracing::info!(?register_resp);

    // taken
    let username = register_resp.user_id.localpart();
    let resp = client
        .get(format!("/_commune/client/r0/register/available/{username}").as_str())
        .send()
        .await?;

    assert!(resp.status().is_client_error());

    let resp = client
        .get(
            format!(
                "/_commune/client/r0/register/available/{}",
                generate_comforming_localpart()
            )
            .as_str(),
        )
        .send()
        .await?;

    resp.json().await
}

#[tokio::test]
async fn available_test() {
    let client = Env::new().await;

    let resp = available(&client).await.unwrap();

    tracing::info!(?resp);
}
