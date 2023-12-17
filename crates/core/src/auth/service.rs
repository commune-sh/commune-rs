use std::sync::Arc;

use matrix::client::resources::login::Login;
use matrix::Client as MatrixAdminClient;
use rand::Rng;
use redis::AsyncCommands;

use crate::auth::error::AuthErrorCode;
use crate::util::secret::Secret;
use crate::Result;

const VERIFICATION_CODE_SECS: u64 = 60 * 5;
const VERIFICATION_CODE_PREFIX: &str = "commune::verification_code::";

pub struct LoginCredentials {
    pub username: String,
    pub password: Secret,
}

pub struct LoginCredentialsResponse {
    pub access_token: Secret,
}

pub struct AuthService {
    admin: Arc<MatrixAdminClient>,
    redis: Arc<redis::Client>,
}

impl AuthService {
    pub fn new(admin: Arc<MatrixAdminClient>, redis: Arc<redis::Client>) -> Self {
        Self { admin, redis }
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

    pub async fn create_verification_code(&self, session: String) -> Result<u32> {
        let mut rng = rand::thread_rng();
        let mut conn = self.redis.get_async_connection().await.map_err(|err| {
            tracing::error!(?err, "Failed to get Redis connection");
            AuthErrorCode::RedisConnectionError(err)
        })?;
        let code: u32 = rng.gen_range(10_000..99_999);

        conn.set_ex::<String, u32, u64>(
            format!("{}{}", VERIFICATION_CODE_PREFIX, session),
            code,
            VERIFICATION_CODE_SECS,
        );

        Ok(code)
    }
}
