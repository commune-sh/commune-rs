pub mod root;
pub mod state;
pub mod redact;
pub mod vote;

use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::router::middleware::auth;

pub struct Legacy;

impl Legacy {
    pub fn routes() -> Router {
        let unimplemented = || async {};

        // should we use PUT? this removes the burden of POST reloading the frontend by default
        let protected = Router::new()
            .route("/", post(root::handler))
            .route("/state", post(state::handler))
            // these routes should be merged
            .route("/redact", post(redact::handler))
            .route("/redact/reaction", post(redact::handler))
            // problematic example of POST usage
            .route("/upvote/:id", post(vote::up))
            .route("/downvote/:id", post(vote::down))
            .route_layer(middleware::from_fn(auth));

        // blocked until we figured out how to serialize the full event
        let event = Router::new()
            .route("/", get(unimplemented))
            .route("/thread", get(unimplemented))
            .route("/replies", get(unimplemented))
            .merge(protected);

        let events = Router::new()
            .route("/", get(unimplemented))
            .route("/:room", get(unimplemented));

        Router::new()
            .nest("/event", event)
            .nest("/events", events)
    }
}

