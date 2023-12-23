use std::sync::Arc;

use matrix::client::resources::login::Login;
use matrix::Client as MatrixAdminClient;
use rand::distributions::{Alphanumeric, DistString};
use rand::SeedableRng;
use redis::AsyncCommands;

use crate::auth::error::AuthErrorCode;
use crate::util::secret::Secret;
use crate::Result;

/// TTL for the verification code in Redis
const VERIFICATION_CODE_SECS: u64 = 60 * 5;

/// Prefix for the verification code key in Redis
const VERIFICATION_CODE_PREFIX: &str = "commune::verification_code::";

/// Quantity of elements in each of the parts conforming the verification code,
/// should be a even nember in order to have no remainder when dividing the
/// capacity of the verification code string.
const VERIFICATION_CODE_CHAR: usize = 4;

/// Quantity of parts conforming the verification code, should be a even number
/// in order to have no remainder when dividing the capacity of the verification
/// code string.
const VERIFICATION_CODE_PART: usize = 3;

/// Capacity of the verification code string
const VERIFICATION_CODE_CAPY: usize =
    (VERIFICATION_CODE_PART * VERIFICATION_CODE_CHAR) + VERIFICATION_CODE_PART;

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

    pub async fn create_verification_code(&self, session: String) -> Result<Secret> {
        let mut conn = self.redis.get_async_connection().await.map_err(|err| {
            tracing::error!(?err, "Failed to get Redis connection");
            AuthErrorCode::RedisConnectionError(err)
        })?;
        let code = Self::generate_verification_code();

        conn.set_ex::<String, String, _>(
            format!("{}{}", VERIFICATION_CODE_PREFIX, session),
            code.to_string(),
            VERIFICATION_CODE_SECS,
        )
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to set verification code in Redis");
            AuthErrorCode::RedisConnectionError(err)
        })?;

        Ok(code)
    }

    fn generate_verification_code() -> Secret {
        let mut out = String::with_capacity(VERIFICATION_CODE_CAPY - VERIFICATION_CODE_PART);
        let mut rng = rand::prelude::StdRng::from_entropy();

        Alphanumeric.append_string(
            &mut rng,
            &mut out,
            VERIFICATION_CODE_CAPY - VERIFICATION_CODE_PART,
        );

        Secret::from(
            format!("{}-{}-{}", &out[0..=3], &out[4..=7], &out[8..=11]).to_ascii_lowercase(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::AuthService;

    #[test]
    fn codes_are_never_repeated() {
        let codes = (1..50)
            .map(|_| AuthService::generate_verification_code().to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            codes.len(),
            codes.iter().collect::<std::collections::HashSet<_>>().len()
        );
    }
}
