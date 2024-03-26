pub mod get {
    use matrix::{client::profile::display_name::get::*, ruma_common::OwnedUserId};

    use crate::{commune, error::Result};

    pub async fn service(user_id: impl Into<OwnedUserId>) -> Result<Response> {
        let req = Request::new(user_id.into());

        commune()
            .send_matrix_request(req, None)
            .await
            .map_err(Into::into)
    }
}

pub mod update {
    use matrix::client::{account::whoami, profile::display_name::update::*};

    use crate::{commune, error::Result};

    pub async fn service(
        access_token: impl AsRef<str>,
        display_name: impl Into<String>,
    ) -> Result<Response> {
        let req = whoami::Request::new();

        let whoami::Response { user_id, .. } = commune()
            .send_matrix_request(req, Some(access_token.as_ref()))
            .await?;

        let req = Request::new(user_id, display_name.into());

        commune()
            .send_matrix_request(req, Some(access_token.as_ref()))
            .await
            .map_err(Into::into)
    }
}
