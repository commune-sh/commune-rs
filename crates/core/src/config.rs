use std::sync::RwLock;

use figment::{
    providers::{Format, Toml},
    Figment,
};
use matrix::ruma_common::OwnedServerName;
use serde::Deserialize;
use url::Url;

use crate::util::secret::Secret;

static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

#[derive(Debug, Deserialize)]
pub struct Config {
    pub registration_verification: bool,

    pub matrix: Matrix,
    pub mail: SMTP,
}

#[derive(Debug, Deserialize)]
pub struct SMTP {
    pub host: OwnedServerName,
    pub ports: [u16; 4],
    pub username: Option<String>,
    pub password: Secret,
    pub tls: bool,
}

#[derive(Debug, Deserialize)]
pub struct Matrix {
    pub host: Url,
    pub server_name: OwnedServerName,
    pub admin_token: Secret,
    pub shared_registration_secret: Secret,
}

pub fn config() -> &'static Config {
    // CONFIG.unwrap_or_else(|| {
    //     Figment::new()
    //         .merge(Toml::file(
    //             std::env::var("COMMUNE_CONFIG").unwrap_or("commune.toml".to_owned()),
    //         ))
    //         .extract()
    //         .expect("could not extract config")
    // })
}

pub fn extract() -> Result<(), figment::Error> {
    let mut config = CONFIG.write().unwrap();

    *config = Some(
        Figment::new()
            .merge(Toml::file(
                std::env::var("COMMUNE_CONFIG").unwrap_or("commune.toml".to_owned()),
            ))
            .extract()?,
    );

    Ok(())
}
