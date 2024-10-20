use axum::{
    extract::{FromRequest, Path},
    response::IntoResponse,
    RequestExt as _, RequestPartsExt as _,
};
use http_body_util::{BodyExt as _, Collected};
use ruma::api::{IncomingRequest, OutgoingResponse};

use std::ops::Deref;

use crate::Error;

/// A wrapper to convert an **A**xum request to **R**uma data
pub(crate) struct Ar<T> {
    /// The Ruma type to deserialize the request into
    pub(crate) inner: T,
}

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
        todo!()
    }
}
