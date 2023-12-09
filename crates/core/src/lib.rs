pub mod account;
pub mod auth;
pub mod error;
pub mod mail;
pub mod util;

pub use error::{Error, HttpStatusCode, Result};
use mail::service::MailService;

use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

use matrix::Client as MatrixAdminClient;

use self::account::service::AccountService;
use self::auth::service::AuthService;

pub mod env {
    pub const COMMUNE_SYNAPSE_HOST: &str = "COMMUNE_SYNAPSE_HOST";
    pub const COMMUNE_SYNAPSE_ADMIN_TOKEN: &str = "COMMUNE_SYNAPSE_ADMIN_TOKEN";
    pub const COMMUNE_SYNAPSE_SERVER_NAME: &str = "COMMUNE_SYNAPSE_SERVER_NAME";
    pub const COMMUNE_REGISTRATION_SHARED_SECRET: &str = "COMMUNE_REGISTRATION_SHARED_SECRET";

    pub const SMTP_HOST: &str = "SMTP_HOST";
    pub const SMTP_PORT: &str = "SMTP_PORT";

    pub const MAILDEV_INCOMING_USER: &str = "MAILDEV_INCOMING_USER";
    pub const MAILDEV_INCOMING_PASS: &str = "MAILDEV_INCOMING_USER";
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommuneConfig {
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub maildev_incoming_user: Option<String>,
    pub maildev_incoming_pass: Option<String>,
    pub synapse_host: String,
    pub synapse_admin_token: String,
    pub synapse_server_name: String,
    pub synapse_registration_shared_secret: String,
}

impl Default for CommuneConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl CommuneConfig {
    pub fn new() -> Self {
        Self {
            smtp_host: Self::var_opt(env::SMTP_HOST),
            smtp_port: Self::var_opt(env::SMTP_PORT),
            maildev_incoming_user: Self::var_opt(env::MAILDEV_INCOMING_USER),
            maildev_incoming_pass: Self::var_opt(env::MAILDEV_INCOMING_PASS),
            synapse_host: Self::var(env::COMMUNE_SYNAPSE_HOST),
            synapse_admin_token: Self::var(env::COMMUNE_SYNAPSE_ADMIN_TOKEN),
            synapse_server_name: Self::var(env::COMMUNE_SYNAPSE_SERVER_NAME),
            synapse_registration_shared_secret: Self::var(env::COMMUNE_REGISTRATION_SHARED_SECRET),
        }
    }

    fn var(name: &str) -> String {
        std::env::var(name).unwrap_or_else(|_| panic!("{} must be set", name))
    }

    fn var_opt<P: Debug + FromStr>(name: &str) -> Option<P> {
        if let Ok(value) = std::env::var(name) {
            if let Ok(value) = value.parse() {
                return Some(value);
            }

            panic!(
                "Failed to parse {} as {:?}",
                name,
                std::any::type_name::<P>()
            );
        }

        None
    }
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
        let mail = Arc::new(MailService::new(&config));

        Ok(Self {
            account: Arc::new(AccountService::new(
                Arc::clone(&admin_client),
                Arc::clone(&mail),
            )),
            auth: Arc::new(auth),
        })
    }
}
