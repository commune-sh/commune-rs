use http::StatusCode;
use matrix::{
    client::{
        session::register::*,
        uiaa::{AuthType, UiaaRequest, UiaaResponse},
    },
    ruma_client::Error::FromHttpResponse,
    ruma_common::api::error::{FromHttpResponseError, MatrixError, MatrixErrorBody},
};

use crate::{error::Result, util::secret::Secret};

pub async fn service(
    handle: &matrix::Handle,
    username: &str,
    password: &Secret,
) -> Result<Response> {
    let req = Request::new(username, password.inner(), "commune", None);

    let resp = handle.dispatch(None, req).await;

    match resp {
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
                            let req = Request::new(
                                username,
                                password.inner(),
                                "commune",
                                Some(UiaaRequest {
                                    session,
                                    kind: AuthType::Dummy,
                                }),
                            );

                            handle.dispatch(None, req).await.map_err(Into::into)
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

// pub async fn verify_email(
//     handle: matrix::Handle,
//     username: &str,
//     password: Secret,
//     email: &str,
// ) -> Result<AccessToken> {
//     use session::register::*;

//     let verification_code: String = rand::thread_rng()
//         .gen::<[u32; 6]>()
//         .iter()
//         .flat_map(|i| char::from_digit(i % 10, 10))
//         .collect();

//     let req = Request::new(username, password.inner(), "commune beta");

//     let resp: Response = handle.dispatch(None, req).await?;

//     Ok(AccessToken::new(resp.access_token))
// }
