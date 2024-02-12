use std::sync::Arc;

use matrix::{
    client::resources::login::{Login, LoginFlows as LoginFlowsResponse},
    Client as MatrixAdminClient,
};
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{auth::error::AuthErrorCode, util::secret::Secret, Error, Result};

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
        // ???
        .unwrap();

        Ok(LoginCredentialsResponse {
            access_token: Secret::new(login_response.access_token),
        })
    }

    pub async fn get_login_flows(&self) -> Result<LoginFlowsResponse> {
        match Login::get_login_flows(&self.admin).await {
            Ok(flows) => Ok(flows),
            Err(err) => {
                tracing::error!("Failed to get login flows: {}", err);
                Err(Error::Unknown)
            }
        }
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

        let maybe_marshalled_verification_code = conn
            .get::<String, Option<String>>(Self::verification_code_key(session))
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

        if let Some(marshalled_verification_code) = maybe_marshalled_verification_code {
            let verification_code = VerificationCode::unmarshall(marshalled_verification_code);

            if verification_code.email == email
                && verification_code.code == *code
                && verification_code.session == *session
            {
                return Ok(true);
            }
        }

        tracing::warn!(?session, ?email, "Verification code not found in storge");
        Ok(false)
    }

    pub async fn drop_verification_code(&self, email: &str, session: &Uuid) -> Result<bool> {
        let mut conn = self.redis.get_async_connection().await.map_err(|err| {
            tracing::error!(?err, "Failed to get Redis connection");
            AuthErrorCode::RedisConnectionError(err)
        })?;

        conn.del(Self::verification_code_key(session))
            .await
            .map_err(|err| {
                tracing::error!(
                    ?err,
                    ?session,
                    ?email,
                    "Failed to delete verification code in Redis"
                );
                AuthErrorCode::RedisConnectionError(err)
            })?;

        Ok(true)
    }

    fn verification_code_key(session: &Uuid) -> String {
        format!("{}{}", REDIS_VERIFICATION_CODE_PREFIX, session)
    }
}
