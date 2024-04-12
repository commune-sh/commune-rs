use matrix::{
    client::{
        register::root::*,
        uiaa::{self, Auth, AuthData, AuthType, UiaaResponse},
    },
    ruma_client::Error::FromHttpResponse,
    ruma_common::api::error::FromHttpResponseError,
};

use crate::{commune, error::Error, util::secret::Secret};

pub async fn service(
    username: impl Into<String>,
    password: Secret,
    registration_token: Option<String>,
) -> Result<Response, Error> {
    let username: String = username.into();

    let req = Request::new(
        username.clone(),
        password.inner(),
        Some("commune".to_owned()),
        None,
        None,
    );

    let resp = commune().send_matrix_request(req.clone(), None).await;

    let uiaainfo = match resp {
        Err(FromHttpResponse(FromHttpResponseError::Server(UiaaResponse::Auth(uiaainfo)))) => {
            uiaainfo
        }
        resp => return resp.map_err(Into::into),
    };

    if uiaainfo.flows.contains(&matrix::client::uiaa::AuthFlow {
        stages: vec![AuthType::Dummy],
    }) {
        let req = Request::new(
            username.clone(),
            password.inner(),
            Some("commune".to_owned()),
            None,
            Some(Auth::new(AuthData::Dummy(uiaa::Dummy {}), uiaainfo.session)),
        );

        return commune()
            .send_matrix_request(req.clone(), None)
            .await
            .map_err(Into::into);
    }

    if uiaainfo.flows.contains(&matrix::client::uiaa::AuthFlow {
        stages: vec![AuthType::RegistrationToken, AuthType::Dummy],
    }) {
        let req = Request::new(
            username.clone(),
            password.inner(),
            Some("commune".to_owned()),
            None,
            Some(Auth::new(
                AuthData::RegistrationToken(uiaa::RegistrationToken::new(
                    registration_token.unwrap(),
                )),
                uiaainfo.session.clone(),
            )),
        );

        let resp = commune().send_matrix_request(req.clone(), None).await;

        let uiaainfo = match resp {
            Err(FromHttpResponse(FromHttpResponseError::Server(UiaaResponse::Auth(uiaainfo)))) => {
                uiaainfo
            }
            resp => return resp.map_err(Into::into),
        };

        let req = Request::new(
            username,
            password.inner(),
            Some("commune".to_owned()),
            None,
            Some(Auth::new(
                AuthData::Dummy(uiaa::Dummy {}),
                uiaainfo.session.clone(),
            )),
        );

        return commune()
            .send_matrix_request(req.clone(), None)
            .await
            .map_err(Into::into);
    }

    panic!("{uiaainfo:?}")
}
