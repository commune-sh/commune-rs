//! Module for [User-Interactive Authentication API][uiaa] types.
//!
//! [uiaa]: https://spec.matrix.org/latest/client-server-api/#user-interactive-authentication-api

use ruma_common::{thirdparty::Medium, OwnedSessionId, OwnedUserId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UiaaResponse {
    pub flows: Vec<AuthFlow>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub completed: Vec<AuthType>,

    pub params: Box<serde_json::value::RawValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<OwnedSessionId>,
    // #[serde(flatten, skip_serializing_if = "Option::is_none")]
    // pub auth_error: Option<StandardErrorBod>,
}

/// Ordered list of stages required to complete authentication.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthFlow {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<AuthType>,
}

impl AuthFlow {
    pub fn new(stages: Vec<AuthType>) -> Self {
        Self { stages }
    }
}

/// Information for one authentication stage.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum AuthType {
    /// Password-based authentication (`m.login.password`).
    #[serde(rename = "m.login.password")]
    Password,

    /// Google ReCaptcha 2.0 authentication (`m.login.recaptcha`).
    #[serde(rename = "m.login.recaptcha")]
    ReCaptcha,

    /// Email-based authentication (`m.login.email.identity`).
    #[serde(rename = "m.login.email.identity")]
    EmailIdentity,

    /// Phone number-based authentication (`m.login.msisdn`).
    #[serde(rename = "m.login.msisdn")]
    Msisdn,

    /// SSO-based authentication (`m.login.sso`).
    #[serde(rename = "m.login.sso")]
    Sso,

    /// Dummy authentication (`m.login.dummy`).
    #[serde(rename = "m.login.dummy")]
    Dummy,

    /// Registration token-based authentication (`m.login.registration_token`).
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
    // RegistrationToken(RegistrationToken),

    // Fallback acknowledgement.
    // FallbackAcknowledgement(FallbackAcknowledgement),
}

impl AuthData {
    fn kind(&self) -> AuthType {
        match self {
            AuthData::Password(_) => AuthType::Password,
            AuthData::Dummy(_) => AuthType::Dummy,
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
    pub fn new<S: Into<String>>(user_id: impl Into<OwnedUserId>, password: S) -> Self {
        let user: &UserId = &user_id.into();

        Self {
            identifier: UserIdentifier::User {
                user: user.localpart().to_owned(),
            },
            password: password.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Auth {
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
