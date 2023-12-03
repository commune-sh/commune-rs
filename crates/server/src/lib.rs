use std::net::TcpListener;

use anyhow::Result;

pub mod config;
pub mod router;
pub mod services;

use crate::config::ServerConfig;
use crate::router::make_router;
use crate::services::Services;

pub async fn serve(tcp: TcpListener) -> Result<()> {
    let config = ServerConfig::from_env();
    let services = Services::shared(config).await?;
    let router = make_router();
    let router = router.with_state(services);

    axum::Server::from_tcp(tcp)?
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
