use axum::{response::IntoResponse, Json, RequestExt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct Request {
    #[serde(rename = "transaction_id")]
    txn_id: String,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub(crate) struct Response {}

pub(crate) async fn ping_route(request: axum::extract::Request) -> impl IntoResponse {
    let _ = request
        .extract::<Json<Request>, _>()
        .await
        .map_err(|_error| {});
}
