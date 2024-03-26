use std::net::SocketAddr;

use axum::{
    routing::{get, post, put},
    Router,
};
use tokio::net::TcpListener;

pub mod api;

pub async fn routes() -> Router {
    let router = Router::new()
        .route("/register", post(api::relative::register::handler))
        .route("/register/available/:username", get(api::relative::available::handler))
        .route("/login", post(api::relative::login::handler))
        .route("/logout", post(api::relative::logout::handler))
        .nest(
            "/account",
            Router::new()
                .route("/whoami", get(api::account::whoami::handler))
                .route("/password", put(api::account::password::handler))
                .route("/display_name", put(api::account::display_name::handler))
                .route("/avatar", put(api::account::avatar::handler))
        );

    Router::new().nest("/_commune/client/r0", router)
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
