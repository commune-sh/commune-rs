use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    commune::init();
    let config = &commune::commune().config;

    tracing_subscriber::fmt::init();

    router::serve(config.public_loopback, config.port.unwrap()).await?;

    Ok(())
}
