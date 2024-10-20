use std::future::Future;

use axum::{
    http::Method,
    response::IntoResponse,
    routing::{on, MethodFilter},
    Router,
};
use ruma::api::{IncomingRequest, Metadata};

use crate::api::ruma::Ar;

trait RouterExt {
    fn ruma_route<H, T>(self, handler: H) -> Self
    where
        H: RumaHandler<T>,
        T: 'static;
}

impl RouterExt for Router {
    fn ruma_route<H, T>(self, handler: H) -> Self
    where
        H: RumaHandler<T>,
        T: 'static,
    {
        handler.add_to_router(self)
    }
}

pub(crate) trait RumaHandler<T> {
    // Can't transform to a handler without boxing or relying on the
    // nightly-only impl-trait-in-traits feature. Moving a small amount of
    // extra logic into the trait allows bypassing both.
    fn add_to_router(self, router: Router) -> Router;
}

impl<Req, Resp, E, F, Fut> RumaHandler<Ar<Req>> for F
where
    Req: IncomingRequest + Send + 'static,
    Resp: IntoResponse,
    F: FnOnce(Ar<Req>) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Result<Resp, E>> + Send,
    E: IntoResponse,
{
    fn add_to_router(self, router: Router) -> Router {
        let Metadata {
            method, history, ..
        } = Req::METADATA;

        let method_filter = match method {
            Method::DELETE => MethodFilter::DELETE,
            Method::GET => MethodFilter::GET,
            Method::HEAD => MethodFilter::HEAD,
            Method::OPTIONS => MethodFilter::OPTIONS,
            Method::PATCH => MethodFilter::PATCH,
            Method::POST => MethodFilter::POST,
            Method::PUT => MethodFilter::PUT,
            Method::TRACE => MethodFilter::TRACE,
            m => panic!("Unsupported HTTP method: {m:?}"),
        };

        history.all_paths().fold(router, |router, path| {
            let handler = self.clone();

            router.route(
                path,
                on(method_filter, |request: Ar<Req>| async move {
                    handler(request).await
                }),
            )
        })
    }
}
