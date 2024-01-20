use commune_server::router::api::v1::account::root::AccountRegisterPayload;
use commune_server::router::api::v1::account::session::AccountSessionResponse;
use commune_server::router::api::ApiError;
use fake::faker::internet::en::{FreeEmail, Password};
use fake::Fake;
use reqwest::StatusCode;
use scraper::Selector;
use uuid::Uuid;

use commune::util::secret::Secret;
use commune_server::router::api::v1::account::login::{AccountLoginPayload, AccountLoginResponse};
use commune_server::router::api::v1::account::verify_code::{
    AccountVerifyCodePayload, VerifyCodeResponse,
};
use commune_server::router::api::v1::account::verify_code_email::{
    AccountVerifyCodeEmailPayload, VerifyCodeEmailResponse,
};

use crate::tools::http::HttpClient;
use crate::tools::maildev::MailDevClient;

#[tokio::test]
async fn retrieves_session_user_from_token() {
    let http_client = HttpClient::new().await;
    let session = Uuid::new_v4();
    let email: String = FreeEmail().fake();
    let verify_code_pld = AccountVerifyCodePayload {
        email: email.clone(),
        session,
    };
    let verify_code_res = http_client
        .post("/api/v1/account/verify/code")
        .json(&verify_code_pld)
        .send()
        .await;
    let verify_code = verify_code_res.json::<VerifyCodeResponse>().await;

    assert!(verify_code.sent, "should return true for sent");

    let maildev = MailDevClient::new();
    let mail = maildev.latest().await.unwrap().unwrap();
    let html = mail.html();
    let code_sel = Selector::parse("#code").unwrap();
    let mut code_el = html.select(&code_sel);
    let code = code_el.next().unwrap().inner_html();
    let verify_code_email_pld = AccountVerifyCodeEmailPayload {
        email: email.clone(),
        code: Secret::new(code.clone()),
        session,
    };

    let verify_code_res = http_client
        .post("/api/v1/account/verify/code/email")
        .json(&verify_code_email_pld)
        .send()
        .await;
    let verify_code_email = verify_code_res.json::<VerifyCodeEmailResponse>().await;

    assert!(verify_code_email.valid, "should return true for valid");

    let username: String = (10..12).fake();
    let username = username.to_ascii_lowercase();
    let password: String = Password(14..20).fake();
    let request_payload = AccountRegisterPayload {
        username: username.clone(),
        password: password.clone(),
        email: email.clone(),
        code,
        session,
    };
    let response = http_client
        .post("/api/v1/account")
        .json(&request_payload)
        .send()
        .await;

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "should return 201 for successful registration"
    );

    let response = http_client
        .post("/api/v1/account/login")
        .json(&AccountLoginPayload {
            username: username.clone(),
            password,
        })
        .send()
        .await;
    let response_status = response.status();
    let response_payload = response.json::<AccountLoginResponse>().await;

    assert_eq!(
        response_status,
        StatusCode::OK,
        "should return 200 for successful login"
    );
    assert!(!response_payload.access_token.is_empty());

    let session_res = http_client
        .get("/api/v1/account/session")
        .token(response_payload.access_token)
        .send()
        .await;
    let session_res_status = session_res.status();
    let session_res_payload = session_res.json::<AccountSessionResponse>().await;

    assert_eq!(
        session_res_status,
        StatusCode::OK,
        "should return 200 for successful session"
    );
    assert!(session_res_payload
        .credentials
        .username
        .starts_with(&format!("@{}", username)));
    assert_eq!(
        session_res_payload.credentials.email, email,
        "should return email"
    );
    assert!(
        session_res_payload.credentials.verified,
        "should return verified"
    );
    assert!(
        !session_res_payload.credentials.admin,
        "should return admin"
    );
}

#[tokio::test]
async fn kicks_users_with_no_token_specified() {
    let http_client = HttpClient::new().await;
    let session_res = http_client.get("/api/v1/account/session").send().await;
    let session_res_status = session_res.status();
    let session_res_payload = session_res.json::<ApiError>().await;

    assert_eq!(session_res_status, StatusCode::UNAUTHORIZED.as_u16(),);
    assert_eq!(
        session_res_payload.code, "UNAUTHORIZED",
        "should return UNAUTHORIZED"
    );
    assert_eq!(
        session_res_payload.message,
        "You must be authenticated to access this resource",
    );
}
