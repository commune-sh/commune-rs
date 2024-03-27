use commune::util::secret::Secret;
use router::api::account::password::Payload;

use crate::{api::relative::login, env::Env};

pub async fn update_password(client: &Env) -> Result<bool, reqwest::Error> {
    let login_resp = login::login(&client).await.unwrap();

    tracing::info!(?login_resp);

    let resp = client
        .put("/_commune/client/r0/account/password")
        .json(&Payload {
            username: login_resp.user_id.localpart().to_owned(),
            password: Secret::new("verysecure"),
            new_password: Secret::new("notverysecure"),
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
async fn update_password_test() {
    let client = Env::new().await;

    let resp = update_password(&client).await.unwrap();

    tracing::info!(?resp);

    assert_eq!(resp, true);
}
