use std::sync::Arc;

use matrix::client::resources::login::Login;
use matrix::Client as MatrixAdminClient;
use redis::AsyncCommands;
use uuid::Uuid;

use crate::auth::error::AuthErrorCode;
use crate::util::secret::Secret;
use crate::Result;

use super::model::VerificationCode;

/// Prefix for the verification code key in Redis
const REDIS_VERIFICATION_CODE_PREFIX: &str = "commune::verification_code::";

/// TTL for the verification code in Redis
const REDIS_VERIFICATION_CODE_SECS: u64 = 60 * 5;

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

    pub async fn send_verification_code(
        &self,
        email: &str,
        session: &Uuid,
    ) -> Result<VerificationCode> {
        let mut conn = self.redis.get_async_connection().await.map_err(|err| {
            tracing::error!(?err, "Failed to get Redis connection");
            AuthErrorCode::RedisConnectionError(err)
        })?;
        let verif_code = VerificationCode::new(email, session);

        conn.set_ex::<String, String, _>(
            Self::verification_code_key(session),
            verif_code.marshall(),
            REDIS_VERIFICATION_CODE_SECS,
        )
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to set verification code in Redis");
            AuthErrorCode::RedisConnectionError(err)
        })?;

        Ok(verif_code)
    }

    pub async fn check_verification_code(
        &self,
        email: &str,
        session: &Uuid,
        code: &Secret,
    ) -> Result<bool> {
        let mut conn = self.redis.get_async_connection().await.map_err(|err| {
            tracing::error!(?err, "Failed to get Redis connection");
            AuthErrorCode::RedisConnectionError(err)
        })?;

        let marshalled_verification_code = conn
            .get::<String, String>(Self::verification_code_key(session))
            .await
            .map_err(|err| {
                tracing::error!(
                    ?err,
                    ?session,
                    ?email,
                    "Failed to get verification code in Redis"
                );
                AuthErrorCode::RedisConnectionError(err)
            })?;
        let verification_code = VerificationCode::unmarshall(marshalled_verification_code);

        if verification_code.email == email
            && verification_code.code == *code
            && verification_code.session == *session
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn verification_code_key(session: &Uuid) -> String {
        format!("{}{}", REDIS_VERIFICATION_CODE_PREFIX, session)
    }
}
