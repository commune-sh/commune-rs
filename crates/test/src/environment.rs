use std::env::var;

use matrix::admin::Client;

const COMMUNE_REGISTRATION_SHARED_SECRET: &str = "COMMUNE_REGISTRATION_SHARED_SECRET";

pub struct Environment {
    pub client: Client,
    pub registration_shared_secret: String,
}

impl Environment {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let client = Client::new("http://localhost:8008").unwrap();
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
