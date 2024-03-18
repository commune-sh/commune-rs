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
