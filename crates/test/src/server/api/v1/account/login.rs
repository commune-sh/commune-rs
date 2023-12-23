use commune::util::secret::Secret;
use reqwest::StatusCode;
use uuid::Uuid;

use commune::account::service::CreateAccountDto;
use commune_server::router::api::v1::account::login::{AccountLoginPayload, AccountLoginResponse};

use crate::tools::environment::Environment;
use crate::tools::http::HttpClient;

#[tokio::test]
async fn logs_into_account() {
    let environment = Environment::new().await;

    let username = "lucy".to_string();
    let password = "P@ssW0Rd$".to_string();

    environment
        .commune
        .account
        .register(CreateAccountDto {
            username: username.clone(),
            password: password.clone().into(),
            email: "lucyinthesky@gmail.com".to_string(),
            code: Secret::new("1234"),
            session: Uuid::new_v4(),
        })
        .await
        .unwrap();

    let http_client = HttpClient::new().await;

    let response = http_client
        .post("/api/v1/account/login")
        .json(&AccountLoginPayload { username, password })
        .send()
        .await;
    let response_status = response.status();
    let response_payload = response.json::<AccountLoginResponse>().await;

    assert_eq!(
        response_status,
        StatusCode::OK,
        "should return 200 for successful login"
    );
    assert!(!response_payload.access_token.is_empty(),)
}
