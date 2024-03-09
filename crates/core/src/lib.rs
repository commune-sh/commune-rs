//! This library deals with our core logic, such as authorizing user
//! interactions, forwarding regular events and constructing custom requests.

// pub mod auth;
pub mod error;
pub mod config;
pub mod mail;
// pub mod room;
// pub mod session;
pub mod util;

use std::sync::Arc;

pub(crate) use error::Result;

use tokio::sync::OnceCell;
use util::secret::Secret;

pub struct Commune {
    pub admin_token: Arc<Secret>,
    pub handle: Arc<matrix::Handle>,
}

impl Commune {
    pub fn new() -> Result<Self> {
        let config = CONFIG
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
