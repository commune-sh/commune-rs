use std::env::var;

use commune::{Commune, CommuneConfig};
use commune_server::config::{
    COMMUNE_REGISTRATION_SHARED_SECRET, COMMUNE_SYNAPSE_ADMIN_TOKEN, COMMUNE_SYNAPSE_HOST,
    COMMUNE_SYNAPSE_SERVER_NAME,
};
use matrix::Client;

pub struct Environment {
    pub client: Client,
    pub commune: Commune,
    pub config: CommuneConfig,
}

impl Environment {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();

        let synapse_host = Self::env_var(COMMUNE_SYNAPSE_HOST);
        let synapse_server_name = Self::env_var(COMMUNE_SYNAPSE_SERVER_NAME);
        let synapse_admin_token = Self::env_var(COMMUNE_SYNAPSE_ADMIN_TOKEN);
        let synapse_registration_shared_secret = Self::env_var(COMMUNE_REGISTRATION_SHARED_SECRET);
        let client = Client::new(synapse_host.clone(), synapse_server_name.clone()).unwrap();

        let config = CommuneConfig {
            synapse_host,
            synapse_admin_token,
            synapse_server_name,
            synapse_registration_shared_secret,
        };
        let commune = Commune::new(config.clone()).await.unwrap();

        Self {
            client,
            commune,
            config,
        }
    }

    pub fn env_var(name: &str) -> String {
        var(name).unwrap_or_else(|_| panic!("Missing {name} environment variable"))
    }
}
