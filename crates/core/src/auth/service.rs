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
