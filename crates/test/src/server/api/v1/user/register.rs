use reqwest::StatusCode;

use commune_server::router::api::v1::user::register::{UserRegisterPayload, UserRegisterResponse};

use crate::tools::http::HttpClient;

#[tokio::test]
async fn register_account_with_success() {
    let http_client = HttpClient::new().await;
    let request_payload = UserRegisterPayload {
        username: String::from("john_wick"),
        password: String::from("P@ssW0Rd$"),
        email: String::from("donttrythisathome@gmail.com"),
    };
    let response = http_client
        .post("/api/v1/user/register")
        .json(&request_payload)
        .send()
        .await;
    let response_status = response.status();
    let response_payload = response.json::<UserRegisterResponse>().await;

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
