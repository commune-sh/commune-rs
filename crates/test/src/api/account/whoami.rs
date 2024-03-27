use matrix::client::account::whoami::*;

use crate::{api::relative::login, env::Env};

pub async fn whoami(client: &Env) -> Result<Response, reqwest::Error> {
    let login_resp = login::login(&client).await.unwrap();

    tracing::info!(?login_resp);

    let resp = client
        .get("/_commune/client/r0/account/whoami")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", &login_resp.access_token),
        )
        .send()
        .await?;

    let json = resp.json::<Response>().await?;

    assert_eq!(login_resp.user_id, json.user_id);

    Ok(json)
}

#[tokio::test]
async fn whoami_test() {
    let client = Env::new().await;

    let resp = whoami(&client).await.unwrap();

    tracing::info!(?resp);
}
