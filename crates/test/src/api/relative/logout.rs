use matrix::client::logout::root::*;

use crate::{api::relative::login, env::Env};

pub async fn logout(client: &Env) -> Result<Response, reqwest::Error> {
    let login_resp = login::login(&client).await.unwrap();

    tracing::info!(?login_resp);

    let resp = client
        .post("/_commune/client/r0/logout")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", &login_resp.access_token),
        )
        .send()
        .await
        .unwrap();

    resp.json::<Response>().await
}

#[tokio::test]
async fn logout_test() {
    let client = Env::new().await;

    let resp = logout(&client).await.unwrap();

    tracing::info!(?resp);
}
