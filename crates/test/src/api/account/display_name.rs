use router::api::account::display_name::Payload;

use crate::{api::relative::login, env::Env};

pub async fn update_display_name(client: &Env) -> Result<bool, reqwest::Error> {
    let login_resp = login::login(&client).await.unwrap();

    tracing::info!(?login_resp);

    let resp = client
        .put("/_commune/client/r0/account/display_name")
        .json(&Payload {
            display_name: "awesome display name".to_owned(),
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
async fn update_display_name_test() {
    let client = Env::new().await;

    let resp = update_display_name(&client).await.unwrap();

    tracing::info!(?resp);

    assert_eq!(resp, true);
}
