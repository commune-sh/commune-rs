//! This library deals with our core logic, such as authorizing user
//! interactions, forwarding regular events and constructing custom requests.

pub mod auth;
pub mod error;
pub mod mail;
pub mod room;
pub mod session;
pub mod util;

use std::sync::Arc;

pub use error::{Error, HttpStatusCode, Result};

use figment::{
    providers::{Format, Toml},
    Figment,
};
use matrix::ruma_common::OwnedServerName;
use url::Url;
use util::secret::Secret;

pub struct Config {
    pub smtp_host: Url,
    pub redis_host: Url,
    pub maildev_incoming_user: Option<String>,
    pub maildev_incoming_pass: Option<String>,
    pub synapse_host: Url,
    pub synapse_admin_token: Secret,
    pub synapse_server_name: OwnedServerName,
    pub synapse_registration_shared_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        let config = Figment::new().merge(Toml::file(
            std::env::var("COMMUNE_CONFIG").unwrap_or("commune.toml".to_owned()),
        ));

        match config.extract::<Config>() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Could not extract config: {e}");

                std::process::exit(1);
            }
        }
    }
}

pub struct Commune {
    handle: Arc<matrix::Handle>,
    admin_token: Arc<Secret>,
}

impl Commune {
    pub fn new(config: Config) -> Result<Self> {
        let handle = matrix::Handle::new(&config.synapse_host);

        Ok(Self {
            handle: Arc::new(handle),
            admin_token: Arc::new(config.synapse_admin_token),
        })
    }
}

//         admin
//             .set_token(&config.synapse_admin_token)
//             .map_err(|err| {
//                 tracing::error!(?err, "Failed to set admin token");
//                 Error::Startup(err.to_string())
//             })?;

//         let redis = {
//             let client =
// redis::Client::open(config.redis_host.to_string()).map_err(|err| {
//                 tracing::error!(?err, host=%config.redis_host.to_string(),
// "Failed to open connection to Redis");                 
// Error::Startup(err.to_string())             })?;
//             let mut conn = client.get_async_connection().await.map_err(|err|
// {                 tracing::error!(?err, host=%config.redis_host.to_string(),
// "Failed to get connection to Redis");                 
// Error::Startup(err.to_string())             })?;

//             redis::cmd("PING").query_async(&mut conn).await.map_err(|err| {
//                 tracing::error!(?err, host=%config.redis_host.to_string(),
// "Failed to ping Redis");                 Error::Startup(err.to_string())
//             })?;

//             tracing::info!(host=%config.redis_host.to_string(), "Connected to
// Redis");

//             Arc::new(client)
//         };

//         let admin_client = Arc::new(admin);
//         let auth = Arc::new(AuthService::new(
//             Arc::clone(&admin_client),
//             Arc::clone(&redis),
//         ));
//         let mail = Arc::new(MailService::new(&config));
//         let account = AccountService::new(
//             Arc::clone(&admin_client),
//             Arc::clone(&auth),
//             Arc::clone(&mail),
//         );
//         let room = RoomService::new(Arc::clone(&admin_client));

//         Ok(Self {
//             account: Arc::new(account),
//             auth,
//             room: Arc::new(room),
//         })
//     }
// }
