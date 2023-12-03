pub mod account;
pub mod auth;
pub mod error;
pub mod util;

pub use error::{Error, HttpStatusCode, Result};

use std::fmt::Debug;
use std::sync::Arc;

use matrix::Client as MatrixAdminClient;

use self::account::service::AccountService;
use self::auth::service::AuthService;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommuneConfig {
    pub synapse_host: String,
    pub synapse_admin_token: String,
    pub synapse_server_name: String,
    pub synapse_registration_shared_secret: String,
}

pub struct Commune {
    pub account: Arc<AccountService>,
    pub auth: Arc<AuthService>,
}

impl Commune {
    pub async fn new<C: Into<CommuneConfig>>(config: C) -> Result<Self> {
        let config: CommuneConfig = config.into();
        let mut admin = MatrixAdminClient::new(&config.synapse_host, &config.synapse_server_name)
            .map_err(|err| {
            tracing::error!(?err, "Failed to create admin client");
            Error::Unknown
        })?;

        admin
            .set_token(&config.synapse_admin_token)
            .map_err(|err| {
                tracing::error!(?err, "Failed to set admin token");
                Error::Unknown
            })?;

        let admin_client = Arc::new(admin);
        let auth = AuthService::new(Arc::clone(&admin_client));

        Ok(Self {
            account: Arc::new(AccountService::new(Arc::clone(&admin_client))),
            auth: Arc::new(auth),
        })
    }
}
