use commune::CommuneConfig;

pub struct ServerConfig {
    pub commune_config: CommuneConfig,
}

impl ServerConfig {
    pub fn from_env() -> ServerConfig {
        ServerConfig {
            commune_config: CommuneConfig::new(),
        }
    }
}
