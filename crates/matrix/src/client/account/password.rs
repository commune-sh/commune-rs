use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedUserId,
};
use serde::Serialize;

use crate::client::uiaa::{self, Auth, AuthData};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/account/password",
    }
};

#[request(error = crate::Error)]
#[derive(Serialize)]
pub struct Request {
    pub auth: Auth,

    pub logout_devices: bool,

    pub new_password: String,
}

impl Request {
    pub fn new(new_password: String) -> Self {
        Self {
            auth: Auth::new(AuthData::Dummy(uiaa::Dummy {}), None),
            logout_devices: false,
            new_password,
        }
    }

    pub fn with_password(
        mut self,
        user_id: OwnedUserId,
        password: String,
        // auth_session: Option<impl Into<OwnedSessionId>>,
    ) -> Self {
        self.auth = Auth::new(
            AuthData::Password(uiaa::Password::new(user_id, password)),
            // auth_session.map(Into::into),
            None,
        );

        self
    }
}

#[response(error = crate::Error)]
pub struct Response {}
