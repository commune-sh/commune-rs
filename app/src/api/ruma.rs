use axum::{
    body::Body,
    extract::{FromRequest, Path},
    response::IntoResponse,
    RequestExt as _, RequestPartsExt as _,
};
use bytes::BytesMut;
use http::StatusCode;
use http_body_util::{BodyExt as _, Collected};
use ruma::api::{IncomingRequest, OutgoingResponse};

use std::ops::Deref;

use crate::Error;

/// A wrapper to convert an **A**xum request to **R**uma data
pub(crate) struct Ar<T> {
    /// The Ruma type to deserialize the request into
    pub(crate) inner: T,
}

/// Automatically coerces `Ar<T>` to `T`,
/// allowing `T`'s methods and fields to be accessed without unpacking `Ar<T>`.
impl<T> Deref for Ar<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A wrapper to convert **R**uma data to an **A**xum response
#[derive(Clone)]
pub(crate) struct Ra<T>(pub(crate) T);

impl<T> From<T> for Ra<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

#[axum::async_trait]
impl<T, S> FromRequest<S> for Ar<T>
where
    T: IncomingRequest,
{
    type Rejection = Error;

    async fn from_request(
        request: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = request.with_limited_body().into_parts();

        let body = body
            .collect()
            .await
            .map(Collected::to_bytes)
            .map_err(|error| Error::Todo(()))?;

        let path_params: Path<Vec<String>> = parts.extract().await?;

        let request = axum::extract::Request::from_parts(parts, body);

        let inner =
            T::try_from_http_request(request, &path_params).map_err(|_error| Error::Todo(()))?;

        Ok(Ar { inner })
    }
}

impl<T: OutgoingResponse> IntoResponse for Ra<T> {
    fn into_response(self) -> axum::response::Response {
        let Ra(request) = self;

        let Ok(response) = request.try_into_http_response() else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };
        response
            .map(|bytes| Body::from(BytesMut::freeze(bytes)))
            .into_response()
    }
}

/// Handler declaration macro.
///
/// - `$( $path:ident )::*` matches one or more [identifier](https://veykril.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#ident) fragment specifiers separated by `::`.
/// This is used for imports such as `appservice::ping`.
#[macro_export]
macro_rules! ruma_route {
    ($endpoint:ident => |$state:ident, $req:ident| $block:block) => {
        use axum::extract::State;

        use $crate::api::ruma::{Ar, Ra};

        pub(crate) async fn $endpoint(
            #[allow(unused_variables)]
            State($state): State<$crate::router::State>,
            #[allow(unused_variables)]
            $req: Ar<$endpoint::Request>,
        ) -> Result<
            Ra<$endpoint::Response>, $crate::Error
        > $block
    };
    ($( $path:ident )::+ = $endpoint:ident @ $ver:ident =>
        |$state:ident, $req:ident| $block:block) => {
            use axum::extract::State;
            use ruma::api::$($path::)+$endpoint;

            use $crate::api::ruma::{Ar, Ra};

            pub(crate) async fn $endpoint(
                #[allow(unused_variables)]
                State($state): State<$crate::router::State>,
                #[allow(unused_variables)]
                $req: Ar<$endpoint::$ver::Request>,
            ) -> Result<
                Ra<$endpoint::$ver::Response>, $crate::Error
            > $block
        };
}
