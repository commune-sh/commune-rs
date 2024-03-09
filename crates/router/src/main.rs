use std::net::SocketAddr;

use anyhow::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // if dotenv().ok().is_some() {
    //     println!("Loaded variables from .env file");
    // }

    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let tcp = TcpListener::bind(addr).await?;

    tracing::info!("Listening on {}", addr);

    router::serve(tcp).await?;

    Ok(())
}
