use crate::{api::relative::login, env::Env};

pub async fn logout(client: &Env) -> Result<bool, reqwest::Error> {
    let login_resp = login::login(&client).await.unwrap();

    tracing::info!(?login_resp);

    let resp = client
        .post("/_commune/client/r0/logout")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", &login_resp.access_token),
        )
        .send()
        .await?;

    // TODO: use `/whoami` to confirm access token is invalid
    Ok(resp.status().is_success())
}

#[tokio::test]
async fn logout_test() {
    let client = Env::new().await;

    let resp = logout(&client).await.unwrap();

    tracing::info!(?resp);

    assert_eq!(resp, true);
}
