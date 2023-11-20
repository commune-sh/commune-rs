use std::env::var;

use matrix::admin::Client;

const COMMUNE_REGISTRATION_SHARED_SECRET: &str = "COMMUNE_REGISTRATION_SHARED_SECRET";
const COMMUNE_SYNAPSE_HOST: &str = "COMMUNE_SYNAPSE_HOST";

pub struct Environment {
    pub client: Client,
    pub registration_shared_secret: String,
}

impl Environment {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let synapse_host = Self::env_var(COMMUNE_SYNAPSE_HOST);
        let client = Client::new(synapse_host).unwrap();
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
