use matrix::ruma_common::OwnedMxcUri;
use router::api::account::avatar::Payload;

use crate::{api::relative::login, env::Env};

pub async fn update_avatar(client: &Env) -> Result<bool, reqwest::Error> {
    let login_resp = login::login(&client).await.unwrap();

    tracing::info!(?login_resp);

    let resp = client
        .put("/_commune/client/r0/account/avatar")
        .json(&Payload {
            mxc_uri: OwnedMxcUri::try_from("mxc://example.org/SEsfnsuifSDFSSEF").unwrap(),
        })
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", &login_resp.access_token),
        )
        .send()
        .await?;

    Ok(resp.status().is_success())
}

#[tokio::test]
async fn update_avatar_test() {
    let client = Env::new().await;

    let resp = update_avatar(&client).await.unwrap();

    tracing::info!(?resp);

    assert_eq!(resp, true);
}
