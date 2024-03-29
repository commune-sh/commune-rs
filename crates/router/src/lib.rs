use std::net::SocketAddr;

use axum::{
    routing::{get, post, put},
    Router,
};
use tokio::net::TcpListener;

pub mod api;

pub async fn routes() -> Router {
    let router = Router::new()
        .route("/login", post(api::relative::login::handler))
        .route("/logout", post(api::relative::logout::handler))
        .nest(
            "/register",
            Router::new()
                .route("/", post(api::register::root::handler))
                .route(
                    "/available/:username",
                    get(api::register::available::handler),
                )
                .route("/email/:email", get(api::register::email::handler)),
        )
        .nest(
            "/account",
            Router::new()
                .route("/whoami", get(api::account::whoami::handler))
                .route("/password", put(api::account::password::handler))
                .route("/display_name", put(api::account::display_name::handler))
                .route("/avatar", put(api::account::avatar::handler)),
        )
        .nest(
            "/direct",
            Router::new().route("/", post(api::direct::create::handler)),
        )
        .nest(
            "/spaces",
            Router::new()
                .route("/", post(api::spaces::create::handler))
                .route(
                    "/:space_id/channels",
                    post(api::spaces::channels::create::handler),
                ),
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
