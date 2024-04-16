use matrix::{
    client::{login, profile::avatar_url::get},
    ruma_common::OwnedMxcUri,
};
use url::form_urlencoded::byte_serialize;

use crate::env::Env;

pub async fn update_avatar(
    client: &Env,
    login_resp: login::Response,
) -> Result<(Payload, reqwest::Response), reqwest::Error> {
    let req = Payload {
        mxc_uri: OwnedMxcUri::try_from("mxc://example.org/SEsfnsuifSDFSSEF").unwrap(),
    };
    let resp = client
        .put("/_commune/client/r0/account/avatar")
        .json(&req)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", &login_resp.access_token),
        )
        .send()
        .await
        .map_err(Into::into)?;

    Ok((req, resp))
}

pub async fn get_avatar(
    client: &Env,
    login_resp: login::Response,
) -> Result<((), get::Response), reqwest::Error> {
    let resp = client
        .get(&format!(
            "/_commune/client/r0/account/{}/avatar",
            byte_serialize(login_resp.user_id.as_bytes()).collect::<String>()
        ))
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", &login_resp.access_token),
        )
        .send()
        .await?;

    tracing::info!(?resp);

    Ok(((), resp.json().await?))
}

#[tokio::test]
async fn avatar_test() {
    let client = Env::new().await;

    let login_resp = crate::api::relative::login::login(&client).await.unwrap();
    tracing::info!(?login_resp);

    let (update_req, update_resp) = update_avatar(&client, login_resp.clone()).await.unwrap();
    tracing::info!(?update_resp);

    let (_, get_resp) = get_avatar(&client, login_resp.clone()).await.unwrap();
    tracing::info!(?get_resp);

    assert_eq!(update_req.mxc_uri, get_resp.avatar_url);
}
