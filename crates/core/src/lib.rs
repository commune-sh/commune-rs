//! This library deals with our core logic, such as authorizing user interactions,
//! forwarding regular events and constructing custom requests.

pub mod session;
pub mod auth;
pub mod error;
pub mod mail;
pub mod room;
pub mod util;

pub use error::{Error, HttpStatusCode, Result};

use mail::service::MailService;
use room::service::RoomService;
use tokio::sync::mpsc::Receiver;
use url::Url;

use std::{fmt::Debug, str::FromStr, sync::Arc};

pub mod env {
    pub const COMMUNE_SYNAPSE_HOST: &str = "COMMUNE_SYNAPSE_HOST";
    pub const COMMUNE_SYNAPSE_ADMIN_TOKEN: &str = "COMMUNE_SYNAPSE_ADMIN_TOKEN";
    pub const COMMUNE_SYNAPSE_SERVER_NAME: &str = "COMMUNE_SYNAPSE_SERVER_NAME";
    pub const COMMUNE_REGISTRATION_SHARED_SECRET: &str = "COMMUNE_REGISTRATION_SHARED_SECRET";
    pub const REDIS_HOST: &str = "REDIS_HOST";
    pub const SMTP_HOST: &str = "SMTP_HOST";
    pub const MAILDEV_INCOMING_USER: &str = "MAILDEV_INCOMING_USER";
    pub const MAILDEV_INCOMING_PASS: &str = "MAILDEV_INCOMING_USER";
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommuneConfig {
    pub smtp_host: Url,
    pub redis_host: Url,
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
            smtp_host: Self::var(env::SMTP_HOST),
            redis_host: Self::var(env::REDIS_HOST),
            maildev_incoming_user: Self::var_opt(env::MAILDEV_INCOMING_USER),
            maildev_incoming_pass: Self::var_opt(env::MAILDEV_INCOMING_PASS),
            synapse_host: Self::var(env::COMMUNE_SYNAPSE_HOST),
            synapse_admin_token: Self::var(env::COMMUNE_SYNAPSE_ADMIN_TOKEN),
            synapse_server_name: Self::var(env::COMMUNE_SYNAPSE_SERVER_NAME),
            synapse_registration_shared_secret: Self::var(env::COMMUNE_REGISTRATION_SHARED_SECRET),
        }
    }

    fn var<P: Debug + FromStr>(name: &str) -> P {
        if let Ok(value) = std::env::var(name) {
            if let Ok(value) = value.parse() {
                return value;
            }
        }

        panic!(
            "Failed to parse {} as {:?}",
            name,
            std::any::type_name::<P>()
        );
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

}

impl Commune {
    pub async fn new<C: Into<CommuneConfig>>(config: C) -> Result<Self> {
        let config: CommuneConfig = config.into();
        let mut admin = MatrixAdminClient::new(&config.synapse_host, &config.synapse_server_name)
            .map_err(|err| {
            tracing::error!(?err, "Failed to create admin client");
            Error::Startup(err.to_string())
        })?;

        admin
            .set_token(&config.synapse_admin_token)
            .map_err(|err| {
                tracing::error!(?err, "Failed to set admin token");
                Error::Startup(err.to_string())
            })?;

        let redis = {
            let client = redis::Client::open(config.redis_host.to_string()).map_err(|err| {
                tracing::error!(?err, host=%config.redis_host.to_string(), "Failed to open connection to Redis");
                Error::Startup(err.to_string())
            })?;
            let mut conn = client.get_async_connection().await.map_err(|err| {
                tracing::error!(?err, host=%config.redis_host.to_string(), "Failed to get connection to Redis");
                Error::Startup(err.to_string())
            })?;

            redis::cmd("PING").query_async(&mut conn).await.map_err(|err| {
                tracing::error!(?err, host=%config.redis_host.to_string(), "Failed to ping Redis");
                Error::Startup(err.to_string())
            })?;

            tracing::info!(host=%config.redis_host.to_string(), "Connected to Redis");

            Arc::new(client)
        };

        let admin_client = Arc::new(admin);
        let auth = Arc::new(AuthService::new(
            Arc::clone(&admin_client),
            Arc::clone(&redis),
        ));
        let mail = Arc::new(MailService::new(&config));
        let account = AccountService::new(
            Arc::clone(&admin_client),
            Arc::clone(&auth),
            Arc::clone(&mail),
        );
        let room = RoomService::new(Arc::clone(&admin_client));

        Ok(Self {
            account: Arc::new(account),
            auth,
            room: Arc::new(room),
        })
    }
}
