use commune::CommuneConfig;

pub const COMMUNE_SYNAPSE_HOST: &str = "COMMUNE_SYNAPSE_HOST";
pub const COMMUNE_SYNAPSE_ADMIN_TOKEN: &str = "COMMUNE_SYNAPSE_ADMIN_TOKEN";
pub const COMMUNE_SYNAPSE_SERVER_NAME: &str = "COMMUNE_SYNAPSE_SERVER_NAME";
pub const COMMUNE_REGISTRATION_SHARED_SECRET: &str = "COMMUNE_REGISTRATION_SHARED_SECRET";

pub struct ServerConfig {
    pub synapse_host: String,
    pub synapse_admin_token: String,
    pub synapse_server_name: String,
    pub synapse_registration_shared_secret: String,
}

impl ServerConfig {
    pub fn from_env() -> ServerConfig {
        ServerConfig {
            synapse_host: Self::var(COMMUNE_SYNAPSE_HOST),
            synapse_admin_token: Self::var(COMMUNE_SYNAPSE_ADMIN_TOKEN),
            synapse_server_name: Self::var(COMMUNE_SYNAPSE_SERVER_NAME),
            synapse_registration_shared_secret: Self::var(COMMUNE_REGISTRATION_SHARED_SECRET),
        }
    }

    fn var(name: &str) -> String {
        std::env::var(name).unwrap_or_else(|_| panic!("{} must be set", name))
    }
}

impl From<ServerConfig> for CommuneConfig {
    fn from(val: ServerConfig) -> Self {
        CommuneConfig {
            synapse_host: val.synapse_host,
            synapse_admin_token: val.synapse_admin_token,
            synapse_server_name: val.synapse_server_name,
            synapse_registration_shared_secret: val.synapse_registration_shared_secret,
        }
    }
}
