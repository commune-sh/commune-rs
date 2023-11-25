use std::net::SocketAddr;

use anyhow::Result;
use dotenv::dotenv;

use commune_server::config::ServerConfig;
use commune_server::router::make_router;
use commune_server::services::Services;

#[tokio::main]
async fn main() -> Result<()> {
    if dotenv().ok().is_some() {
        println!("Loaded variables from .env file");
    }

    tracing_subscriber::fmt::init();

    let config = ServerConfig::from_env();
    let services = Services::shared(config)?;
    let router = make_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!(?addr, "server listening");

    axum::Server::bind(&addr)
        .serve(router.with_state(services).into_make_service())
        .await?;

    Ok(())
}
