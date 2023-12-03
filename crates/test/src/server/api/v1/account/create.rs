use reqwest::StatusCode;

use commune_server::router::api::v1::account::create::{
    AccountRegisterPayload, AccountRegisterResponse,
};

use crate::tools::http::HttpClient;

#[tokio::test]
async fn register_account_with_success() {
    let http_client = HttpClient::new().await;
    let request_payload = AccountRegisterPayload {
        username: String::from("john_wick"),
        password: String::from("P@ssW0Rd$"),
        email: String::from("donttrythisathome@gmail.com"),
    };
    let response = http_client
        .post("/api/v1/account")
        .json(&request_payload)
        .send()
        .await;
    let response_status = response.status();
    let response_payload = response.json::<AccountRegisterResponse>().await;

    assert_eq!(
        response_status,
        StatusCode::CREATED,
        "should return 201 for created"
    );
    assert_eq!(
        request_payload.username, response_payload.username,
        "should return the same username"
    )
}
