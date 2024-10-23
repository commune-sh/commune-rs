use std::future::Future;

use axum::Router;

use crate::api;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) db: sled::Db,
    pub(crate) proxy: (),
}

impl State {
    pub(crate) fn new(db: sled::Db) -> Self {
        Self { db, proxy: () }
    }
}

pub(crate) fn router(db: sled::Db) -> Router<State> {
    let state = State::new(db);

    let router = Router::new()
        .ruma_route(api::ping::send_ping)
        .ruma_route(api::rooms::events::push_events);

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
            $( http::Method::$variant => axum::routing::MethodFilter::$variant, )*
            m => panic!("Unsupported HTTP method: {m:?}"),
        }
    };
}

macro_rules! impl_ruma_handler {
    ( $state:ty ) => {
        #[axum::async_trait]
        #[allow(non_snake_case)]
        impl<Req, Resp, E, F, Fut> RumaHandler<$state, $crate::api::ruma::Ar<Req>> for F
        where
            Req: ruma::api::IncomingRequest + Send + 'static,
            Resp: axum::response::IntoResponse,
            F: FnOnce(axum::extract::State<$state>, $crate::api::ruma::Ar<Req>) -> Fut + Clone + Send + 'static,
            Fut: Future<Output = Result<Resp, E>> + Send,
            E: axum::response::IntoResponse,
        {
            fn add_to_router(self, router: Router<$state>) -> Router<$state> {
                let ruma::api::Metadata {
                    method, history, ..
                } = Req::METADATA;

                let method_filter = into_method_filter!(
                    method => DELETE GET HEAD OPTIONS PATCH POST PUT TRACE
                );

                history.all_paths().fold(router, |router, path| {
                    let handler = self.clone();

                    router.route(
                        path,
                        axum::routing::on(
                            method_filter,
                            |state: axum::extract::State<$state>, request: $crate::api::ruma::Ar<Req>| async move {
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
