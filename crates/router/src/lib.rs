use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

pub mod api;

pub async fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "hello from commune!" }))
        .nest(
            "/_commune/client/r0",
            Router::new()
                .nest(
                    "/register",
                    Router::new()
                        .route("/", post(api::session::register::handler))
                        .route(
                            "/username/:username",
                            get(api::session::username_available::handler),
                        ),
                )
                .route("/login", post(api::session::login::handler))
                .route("/logout", post(api::session::logout::handler)),
        )
}

pub async fn serve(public_loopback: bool, port: u16) -> anyhow::Result<()> {
    let host = match public_loopback {
        true => [0, 0, 0, 0],
        false => [127, 0, 0, 1],
    };

    let addr = SocketAddr::from((host, port));
    let tcp_listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on {}", addr);

    let router = routes().await;

    axum::serve(tcp_listener, router.into_make_service())
        .await
        .map_err(Into::into)
}
