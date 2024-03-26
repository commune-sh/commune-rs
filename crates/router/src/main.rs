use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    commune::init().await;
    let config = &commune::commune().config;

    router::serve(config.public_loopback, config.port.unwrap()).await?;

    Ok(())
}
