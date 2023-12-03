use std::env::var;

use matrix::Client;

const COMMUNE_REGISTRATION_SHARED_SECRET: &str = "COMMUNE_REGISTRATION_SHARED_SECRET";
const COMMUNE_SYNAPSE_HOST: &str = "COMMUNE_SYNAPSE_HOST";
const COMMUNE_SYNAPSE_SERVER_NAME: &str = "COMMUNE_SYNAPSE_SERVER_NAME";

pub struct Environment {
    pub client: Client,
    pub registration_shared_secret: String,
}

impl Environment {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let synapse_host = Self::env_var(COMMUNE_SYNAPSE_HOST);
        let synapse_server_name = Self::env_var(COMMUNE_SYNAPSE_SERVER_NAME);
        let client = Client::new(synapse_host, synapse_server_name).unwrap();
        let registration_shared_secret = Self::env_var(COMMUNE_REGISTRATION_SHARED_SECRET);

        Self {
            client,
            registration_shared_secret,
        }
    }

    pub fn env_var(name: &str) -> String {
        var(name).unwrap_or_else(|_| panic!("Missing {name} environment variable"))
    }
}
