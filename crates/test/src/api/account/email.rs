use crate::{ env::Env};

pub async fn verify_email(client: &Env) -> Result<bool, reqwest::Error> {
    let resp = client
        .get(format!("/_commune/client/r0/register/email/{}", "test@127.0.0.1:1025").as_str())
        .send()
        .await?;

    let token = resp.headers().get("registration-token").map(|t| t.to_str()).unwrap().unwrap();

    tracing::info!(token);

    Ok(resp.status().is_success())
}

#[tokio::test]
async fn verify_email_test() {
    let client = Env::new().await;

    let resp = verify_email(&client).await.unwrap();

    tracing::info!(?resp);

    assert_eq!(resp, true);
}
