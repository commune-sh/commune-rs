//! Module for [User-Interactive Authentication AjI][uiaa] types.
//!
//! [uiaa]: https://spec.matrix.org/latest/client-server-api/#user-interactive-authentication-api

use std::fmt;

use bytes::BufMut;
use ruma_common::{
    api::{error::IntoHttpError, EndpointError, OutgoingResponse},
    thirdparty::Medium,
    OwnedSessionId, OwnedUserId, UserId,
};
use serde::{Deserialize, Serialize};
use serde_json::from_slice as from_json_slice;

use crate::error::StandardErrorBody;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UiaaInfo {
    pub flows: Vec<AuthFlow>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub completed: Vec<AuthType>,

    pub params: Box<serde_json::value::RawValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<OwnedSessionId>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub auth_error: Option<StandardErrorBody>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthFlow {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<AuthType>,
}

impl AuthFlow {
    pub fn new(stages: Vec<AuthType>) -> Self {
        Self { stages }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
pub enum AuthType {
    #[serde(rename = "m.login.password")]
    Password,

    #[serde(rename = "m.login.recaptcha")]
    ReCaptcha,

    #[serde(rename = "m.login.email.identity")]
    EmailIdentity,

    #[serde(rename = "m.login.msisdn")]
    Msisdn,

    #[serde(rename = "m.login.sso")]
    Sso,

    #[serde(rename = "m.login.dummy")]
    Dummy,

    #[serde(rename = "m.login.registration_token")]
    RegistrationToken,
}

#[derive(Clone, Debug, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum AuthData {
    // Password-based authentication (`m.login.password`).
    Password(Password),

    // Google ReCaptcha 2.0 authentication (`m.login.recaptcha`).
    // ReCaptcha(ReCaptcha),

    // Email-based authentication (`m.login.email.identity`).
    // EmailIdentity(EmailIdentity),

    // Phone number-based authentication (`m.login.msisdn`).
    // Msisdn(Msisdn),

    // Dummy authentication (`m.login.dummy`).
    Dummy(Dummy),

    // Registration token-based authentication (`m.login.registration_token`).
    RegistrationToken(RegistrationToken),
    // Fallback acknowledgement.
    // FallbackAcknowledgement(FallbackAcknowledgement),
}

impl AuthData {
    fn kind(&self) -> AuthType {
        match self {
            AuthData::Password(_) => AuthType::Password,
            AuthData::Dummy(_) => AuthType::Dummy,
            AuthData::RegistrationToken(_) => AuthType::RegistrationToken,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "type", rename = "m.login.dummy")]
pub struct Dummy {}

impl Dummy {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename = "m.login.password")]
pub struct Password {
    identifier: UserIdentifier,
    password: String,
}

impl Password {
    pub fn new(user_id: impl Into<OwnedUserId>, password: impl Into<String>) -> Self {
        let user: &UserId = &user_id.into();

        Self {
            identifier: UserIdentifier::User {
                user: user.localpart().to_owned(),
            },
            password: password.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename = "m.login.registration_token")]
pub struct RegistrationToken {
    token: String,
}

impl RegistrationToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Auth {
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<OwnedSessionId>,

    kind: AuthType,

    #[serde(flatten)]
    data: AuthData,
}

impl Auth {
    pub fn new(data: AuthData, session: Option<OwnedSessionId>) -> Self {
        Self {
            session,
            kind: data.kind(),
            data,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum UserIdentifier {
    #[serde(rename = "m.id.user")]
    User { user: String },

    #[serde(rename = "m.id.thirdparty")]
    ThirdParty { medium: Medium, address: String },

    #[serde(rename = "m.id.phone")]
    Phone { country: String, phone: String },
}

#[derive(Clone, Debug)]
#[allow(clippy::exhaustive_enums)]
pub enum UiaaResponse {
    Auth(UiaaInfo),

    Error(crate::Error),
}

impl From<crate::Error> for UiaaResponse {
    fn from(error: crate::Error) -> Self {
        Self::Error(error)
    }
}

impl fmt::Display for UiaaResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auth(_) => write!(f, "User-Interactive Authentication required."),
            Self::Error(err) => write!(f, "{err}"),
        }
    }
}

impl EndpointError for UiaaResponse {
    fn from_http_response<T: AsRef<[u8]>>(response: http::Response<T>) -> Self {
        if response.status() == http::StatusCode::UNAUTHORIZED {
            if let Ok(uiaa_info) = from_json_slice(response.body().as_ref()) {
                return Self::Auth(uiaa_info);
            }
        }

        Self::Error(crate::Error::from_http_response(response))
    }
}

impl std::error::Error for UiaaResponse {}

impl OutgoingResponse for UiaaResponse {
    fn try_into_http_response<T: Default + BufMut>(
        self,
    ) -> Result<http::Response<T>, IntoHttpError> {
        match self {
            UiaaResponse::Auth(authentication_info) => http::Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(&http::StatusCode::UNAUTHORIZED)
                .body(ruma_common::serde::json_to_buf(&authentication_info)?)
                .map_err(Into::into),
            UiaaResponse::Error(error) => error.try_into_http_response(),
        }
    }
}
