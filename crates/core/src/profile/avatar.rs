pub mod get {
    use matrix::{client::profile::avatar_url::get::*, ruma_common::OwnedUserId};

    use crate::{commune, error::Error};

    pub async fn service(user_id: impl Into<OwnedUserId>) -> Result<Response, Error> {
        let req = Request::new(user_id.into());

        commune()
            .send_matrix_request(req, None)
            .await
            .map_err(Into::into)
    }
}

pub mod update {
    use matrix::{
        client::{account::whoami, profile::avatar_url::update::*},
        ruma_common::OwnedMxcUri,
    };

    use crate::{commune, error::Error};

    pub async fn service(
        access_token: impl AsRef<str>,
        mxc_uri: impl Into<OwnedMxcUri>,
    ) -> Result<Response, Error> {
        let req = whoami::Request::new();

        let whoami::Response { user_id, .. } = commune()
            .send_matrix_request(req, Some(access_token.as_ref()))
            .await?;

        let req = Request::new(user_id, mxc_uri.into());

        commune()
            .send_matrix_request(req, Some(access_token.as_ref()))
            .await
            .map_err(Into::into)
    }
}
