use std::sync::Arc;

use matrix::Client as MatrixAdminClient;

use crate::util::secret::Secret;
use crate::Result;

pub struct LoginCredentials {
    pub username: String,
    pub password: Secret,
}

pub struct AuthService {
    admin: Arc<MatrixAdminClient>,
}

impl AuthService {
    pub fn new(admin: Arc<MatrixAdminClient>) -> Self {
        Self { admin }
    }

    pub fn login(&self, _credentials: LoginCredentials) -> Result<()> {
        todo!()
    }
}
