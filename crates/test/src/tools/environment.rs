use commune::{Commune, CommuneConfig};
use matrix::Client;

pub struct Environment {
    pub client: Client,
    pub commune: Commune,
    pub config: CommuneConfig,
}

impl Environment {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();

        let config = CommuneConfig::new();
        let client = Client::new(
            config.synapse_host.clone(),
            config.synapse_server_name.clone(),
        )
        .unwrap();

        let commune = Commune::new(config.clone()).await.unwrap();

        Self {
            client,
            commune,
            config,
        }
    }
}
