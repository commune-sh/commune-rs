use std::sync::Arc;

use matrix::client::resources::login::Login;
use matrix::Client as MatrixAdminClient;

use crate::util::secret::Secret;
use crate::Result;

pub struct LoginCredentials {
    pub username: String,
    pub password: Secret,
}

pub struct LoginCredentialsResponse {
    pub access_token: Secret,
}

pub struct AuthService {
    admin: Arc<MatrixAdminClient>,
}

impl AuthService {
    pub fn new(admin: Arc<MatrixAdminClient>) -> Self {
        Self { admin }
    }

    pub async fn login(&self, credentials: LoginCredentials) -> Result<LoginCredentialsResponse> {
        let login_response = Login::login_credentials(
            &self.admin,
            credentials.username,
            credentials.password.inner(),
        )
        .await
        .unwrap();

        Ok(LoginCredentialsResponse {
            access_token: Secret::new(login_response.access_token),
        })
    }
}
