use matrix::client::login;
use router::api::profile::displayname::Payload;

use crate::{api::profile::get_profile, env::Env};

pub async fn update_displayname(
    client: &Env,
    login_resp: &login::Response,
) -> Result<reqwest::Response, reqwest::Error> {
    let req = Payload {
        displayname: "Some displayname".to_owned(),
    };

    client
        .put("/_commune/client/r0/profile/displayname")
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
async fn update_displayname_test() {
    let client = Env::new().await;

    let login_resp = crate::api::relative::login::login(&client).await.unwrap();
    tracing::info!(?login_resp);

    let update_resp = update_displayname(&client, &login_resp).await.unwrap();
    tracing::info!(?update_resp);

    let profile_resp = get_profile(&client, &login_resp).await.unwrap();
    tracing::info!(?profile_resp);

    assert_eq!(
        profile_resp.displayname,
        Some("Some displayname".to_owned())
    );
}
