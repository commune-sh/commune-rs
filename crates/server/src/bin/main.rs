use std::net::{SocketAddr, TcpListener};

use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    if dotenv().ok().is_some() {
        println!("Loaded variables from .env file");
    }

    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let tcp = TcpListener::bind(addr)?;

    commune_server::serve(tcp).await?;

    Ok(())
}
