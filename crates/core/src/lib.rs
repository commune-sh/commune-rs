//! This library deals with our core logic, such as authorizing user
//! interactions, forwarding regular events and constructing custom requests.

// pub mod auth;
pub mod config;
pub mod error;
pub mod mail;
pub mod session;
// pub mod room;
pub mod util;

use std::sync::{Arc, RwLock};

use config::Config;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

static COMMUNE: RwLock<Option<&'static Commune>> = RwLock::new(None);

pub struct Commune {
    pub config: Config,
    pub handle: Arc<matrix::Handle>,
}

pub fn init() {
    let mut commune = COMMUNE.write().unwrap();

    let config = Figment::new()
        .merge(Toml::file(
            Env::var("COMMUNE_CONFIG").unwrap_or("./commune-example.toml".to_owned()),
        ))
        .extract::<Config>()
        .unwrap();

    let handle = Arc::new(matrix::Handle::new(&config.matrix.host));

    *commune = Some(Box::leak(Box::new(Commune { config, handle })));
}

pub fn commune() -> &'static Commune {
    COMMUNE
        .read()
        .unwrap()
        .expect("commune should be initialized at this point")
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
