use std::future::Future;

use axum::{
    extract::{self, FromRequestParts},
    http::Method,
    response::IntoResponse,
    routing::{on, MethodFilter},
    Router,
};
use ruma::api::{IncomingRequest, Metadata};

use crate::api::{self, ruma::Ar};

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) db: sled::Db,
}

impl State {
    pub(crate) fn new(db: sled::Db) -> Self {
        Self { db }
    }
}

pub(crate) fn router(db: sled::Db) -> Router<State> {
    let state = State::new(db);

    let router = Router::new().ruma_route(api::ping::send_ping);

    router.with_state(state)
}

pub(crate) trait RouterExt<S> {
    fn ruma_route<H, T>(self, handler: H) -> Self
    where
        H: RumaHandler<S, T>,
        T: 'static;
}

impl<S> RouterExt<S> for Router<S> {
    fn ruma_route<H, T>(self, handler: H) -> Self
    where
        H: RumaHandler<S, T>,
        T: 'static,
    {
        handler.add_to_router(self)
    }
}

pub(crate) trait RumaHandler<S, T> {
    // Can't transform to a handler without boxing or relying on the
    // nightly-only impl-trait-in-traits feature. Moving a small amount of
    // extra logic into the trait allows bypassing both.
    fn add_to_router(self, router: Router<S>) -> Router<S>;
}

macro_rules! into_method_filter {
    ( $method:expr => $($variant:ident)* ) => {
        match $method {
            $( Method::$variant => MethodFilter::$variant, )*
            m => panic!("Unsupported HTTP method: {m:?}"),
        }
    };
}

macro_rules! impl_ruma_handler {
    ( $state:ty ) => {
        #[axum::async_trait]
        #[allow(non_snake_case)]
        impl<Req, Resp, E, F, Fut> RumaHandler<$state, Ar<Req>> for F
        where
            Req: IncomingRequest + Send + 'static,
            Resp: IntoResponse,
            F: FnOnce(extract::State<$state>, Ar<Req>) -> Fut + Clone + Send + 'static,
            Fut: Future<Output = Result<Resp, E>> + Send,
            E: IntoResponse,
            // $state: FromRequestParts<$state> + Send + 'static,
            // $( $ty: FromRequestParts<()> + Send + 'static, )*
        {
            fn add_to_router(self, router: Router<$state>) -> Router<$state> {
                let Metadata {
                    method, history, ..
                } = Req::METADATA;

                let method_filter = into_method_filter!(
                    method => DELETE GET HEAD OPTIONS PATCH POST PUT TRACE
                ); 

                history.all_paths().fold(router, |router, path| {
                    let handler = self.clone();

                    router.route(
                        path,
                        on(
                            method_filter,
                            |state: extract::State<$state>, request: Ar<Req>| async move {
                                handler(state, request).await
                            },
                        ),
                    )
                })
            }
        }
    };
}

impl_ruma_handler!(());
impl_ruma_handler!(State);
