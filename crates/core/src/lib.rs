pub mod user;
pub mod util;

use std::fmt::Debug;

use anyhow::Result;

use matrix::admin::Client as MatrixAdminClient;

use self::user::service::UserService;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommuneConfig {
    pub synapse_host: String,
    pub synapse_admin_token: String,
    pub synapse_server_name: String,
    pub synapse_registration_shared_secret: String,
}

pub struct Commune {
    pub user: UserService,
}

impl Commune {
    pub fn new<C: Into<CommuneConfig>>(config: C) -> Result<Self> {
        let config: CommuneConfig = config.into();
        let mut admin = MatrixAdminClient::new(config.synapse_host, config.synapse_server_name)?;

        admin.set_token(config.synapse_admin_token)?;

        Ok(Self {
            user: UserService::new(admin),
        })
    }
}
