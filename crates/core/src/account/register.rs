use http::StatusCode;
use matrix::{
    client::{
        register::root::*,
        uiaa::{Auth, AuthData, AuthType, Dummy, UiaaResponse},
    },
    ruma_client::Error::FromHttpResponse,
    ruma_common::api::error::{FromHttpResponseError, MatrixError, MatrixErrorBody},
};

use crate::{commune, error::Result, util::secret::Secret};

pub async fn service(username: impl Into<String>, password: Secret) -> Result<Response> {
    let req = Request::new(
        username.into(),
        password.inner(),
        Some("commune".to_owned()),
        None,
        None,
    );

    let mut retry_req = req.clone();

    match commune().send_matrix_request(req, None).await {
        Ok(resp) => Ok(resp),
        Err(e) => match e {
            FromHttpResponse(FromHttpResponseError::Server(MatrixError {
                status_code: StatusCode::UNAUTHORIZED,
                body: MatrixErrorBody::Json(ref body),
            })) => {
                let UiaaResponse { flows, session, .. } =
                    serde_json::from_value::<UiaaResponse>(body.clone()).unwrap();

                match flows.as_slice() {
                    [value] => match value.stages.as_slice() {
                        [AuthType::Dummy] => {
                            retry_req.auth = Some(Auth::new(AuthData::Dummy(Dummy {}), session));

                            commune()
                                .send_matrix_request(retry_req, None)
                                .await
                                .map_err(Into::into)
                        }
                        _ => Err(e.into()),
                    },
                    _ => Err(e.into()),
                }
            }

            _ => Err(e.into()),
        },
    }
}
