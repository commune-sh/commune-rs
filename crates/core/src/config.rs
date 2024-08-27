use matrix::ruma_common::OwnedServerName;
use serde::Deserialize;
use url::Url;

use crate::util::secret::Secret;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub registration_verification: bool,
    pub public_loopback: bool,
    pub port: Option<u16>,

    pub allowed_domains: Option<Vec<Url>>,
    pub blocked_domains: Option<Vec<Url>>,

    pub matrix: Matrix,
    pub mail: SMTP,
}

#[derive(Debug, Deserialize)]
pub struct SMTP {
    pub host: Url,
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
