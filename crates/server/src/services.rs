use std::sync::Arc;

use anyhow::Result;

use commune::Commune;

use crate::config::ServerConfig;

pub type SharedServices = Arc<Services>;

pub struct Services {
    pub commune: Commune,
}

impl Services {
    pub async fn new(config: ServerConfig) -> Result<Self> {
        let commune = Commune::new(config.commune_config).await?;

        Ok(Self { commune })
    }

    pub async fn shared(config: ServerConfig) -> Result<SharedServices> {
        Ok(Arc::new(Self::new(config).await?))
    }
}
