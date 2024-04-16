use matrix::{client::login, ruma_common::OwnedMxcUri};
use router::api::profile::avatar_url::Payload;

use crate::{api::profile::get_profile, env::Env};

pub async fn update_avatar_url(
    client: &Env,
    login_resp: &login::Response,
) -> Result<reqwest::Response, reqwest::Error> {
    let req = Payload {
        avatar_url: OwnedMxcUri::try_from("mxc://example.org/SEsfnsuifSDFSSEF").unwrap(),
    };

    client
        .put("/_commune/client/r0/profile/avatar_url")
        .json(&req)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", &login_resp.access_token),
        )
        .send()
        .await
        .map_err(Into::into)
}

#[tokio::test]
async fn update_avatar_url_test() {
    let client = Env::new().await;

    let login_resp = crate::api::relative::login::login(&client).await.unwrap();
    tracing::info!(?login_resp);

    let update_resp = update_avatar_url(&client, &login_resp).await.unwrap();
    tracing::info!(?update_resp);

    let profile_resp = get_profile(&client, &login_resp).await.unwrap();
    tracing::info!(?profile_resp);

    assert_eq!(
        profile_resp.avatar_url,
        Some(OwnedMxcUri::try_from("mxc://example.org/SEsfnsuifSDFSSEF").unwrap())
    );
}
