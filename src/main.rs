use axum::{routing::post, Router};

pub(crate) mod api;

fn main() {
    let _router = Router::<()>::new().route("/ping", post(api::ping_route));
}
