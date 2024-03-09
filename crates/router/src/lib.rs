use anyhow::Result;
use tokio::net::TcpListener;

// pub mod config;
// pub mod router;
// pub mod services;

// use crate::{config::ServerConfig, router::make_router, services::Services};

pub async fn serve(listener: TcpListener) -> Result<()> {
    todo!()

        Router::new()
            .route("/session", get(session::handler))
            .route_layer(middleware::from_fn(auth))
            .route("/", post(root::handler))
            .route("/login", get(login::get))
            .route("/login", post(login::post))
            .route("/login/sso/redirect", get(login::get))
            .route("/email/:email", get(email::handler))
            .nest(
                "/verify",
                Router::new()
                    .route("/code", post(verify_code::handler))
                    .route("/code/email", post(verify_code_email::handler)),
            )

    // let config = ServerConfig::from_env();
    // let services = Services::shared(config).await?;
    // let router = make_router(services);

    // if let Err(err) = axum::serve(listener, router.into_make_service()).await
    // {     tracing::error!(%err, "Failed to initialize the server");
    //     panic!("An error ocurred running the server!");
    // }

    // Ok(())
}
