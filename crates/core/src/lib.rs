pub mod account;
pub mod error;
pub mod util;

pub use error::{Error, HttpStatusCode, Result};

use std::fmt::Debug;

use matrix::admin::Client as MatrixAdminClient;

use self::account::service::AccountService;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommuneConfig {
    pub synapse_host: String,
    pub synapse_admin_token: String,
    pub synapse_server_name: String,
    pub synapse_registration_shared_secret: String,
}

pub struct Commune {
    pub account: AccountService,
}

impl Commune {
    pub fn new<C: Into<CommuneConfig>>(config: C) -> Result<Self> {
        let config: CommuneConfig = config.into();
        let mut admin = MatrixAdminClient::new(config.synapse_host, config.synapse_server_name)
            .map_err(|err| {
                tracing::error!(?err, "Failed to create admin client");
                Error::Unknown
            })?;

        admin.set_token(config.synapse_admin_token).map_err(|err| {
            tracing::error!(?err, "Failed to set admin token");
            Error::Unknown
        })?;

        Ok(Self {
            account: AccountService::new(admin),
        })
    }
}
