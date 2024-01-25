use anyhow::Result;
use tokio::net::TcpListener;

pub mod config;
pub mod router;
pub mod services;

use crate::config::ServerConfig;
use crate::router::make_router;
use crate::services::Services;

pub async fn serve(listener: TcpListener) -> Result<()> {
    let config = ServerConfig::from_env();
    let services = Services::shared(config).await?;
    let router = make_router(services);

    if let Err(err) = axum::serve(listener, router.into_make_service()).await {
        tracing::error!(%err, "Failed to initialize the server");
        panic!("An error ocurred running the server!");
    }

    Ok(())
}
