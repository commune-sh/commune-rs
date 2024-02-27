use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};
use serde::Deserialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: true,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/login",
    }
};

#[request(error = crate::Error)]
pub struct Request {}

#[response(error = crate::Error)]
pub struct Response {
    body: Vec<LoginFlow>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginFlow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_login_token: Option<bool>,

    pub kind: LoginType,
}

#[derive(Clone, Debug, Deserialize)]
pub enum LoginType {
    #[serde(rename = "m.login.password")]
    Password,

    #[serde(rename = "m.login.token")]
    Token,

    #[serde(rename = "m.login.sso")]
    Sso,

    #[serde(rename = "m.login.application_service")]
    ApplicationService,
}
