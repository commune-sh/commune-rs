use std::sync::Arc;

use matrix::client::session;

use crate::{util::secret::Secret, Error};

pub mod error;
pub mod model;
pub mod service;

pub struct Auth {
    handle: Arc<matrix::Handle>,
}

pub type AccessToken = Secret;

impl Auth {
    pub async fn login_with_password(
        &self,
        username: &str,
        password: Secret,
    ) -> Result<AccessToken, Error> {
        use session::login::*;

        let req = Request::new(
            LoginType::Password {
                password: password.inner().to_owned(),
            },
            Some(Identifier::User {
                user: username.to_owned(),
            }),
            "commune beta".to_owned(),
            Some(true),
        );

        let resp: Response = self.handle.dispatch(None, req).await.unwrap().into();

        Ok(AccessToken::new(resp.access_token))
    }

    // pub async fn get_login_flows(&self) -> Result<LoginFlowsResponse> {
    //     match Login::get_login_flows(&self.admin).await {
    //         Ok(flows) => Ok(flows),
    //         Err(err) => {
    //             tracing::error!("Failed to get login flows: {}", err);
    //             Err(Error::Unknown)
    //         }
    //     }
    // }

    // pub async fn send_verification_code(
    //     &self,
    //     email: &str,
    //     session: &Uuid,
    // ) -> Result<VerificationCode> {
    //     let mut conn = self.redis.get_async_connection().await.map_err(|err| {
    //         tracing::error!(?err, "Failed to get Redis connection");
    //         AuthErrorCode::RedisConnectionError(err)
    //     })?;
    //     let verif_code = VerificationCode::new(email, session);

    //     conn.set_ex::<String, String, _>(
    //         Self::verification_code_key(session),
    //         verif_code.marshall(),
    //         REDIS_VERIFICATION_CODE_SECS,
    //     )
    //     .await
    //     .map_err(|err| {
    //         tracing::error!(?err, "Failed to set verification code in Redis");
    //         AuthErrorCode::RedisConnectionError(err)
    //     })?;

    //     Ok(verif_code)
    // }

    // pub async fn check_verification_code(
    //     &self,
    //     email: &str,
    //     session: &Uuid,
    //     code: &Secret,
    // ) -> Result<bool> {
    //     let mut conn = self.redis.get_async_connection().await.map_err(|err| {
    //         tracing::error!(?err, "Failed to get Redis connection");
    //         AuthErrorCode::RedisConnectionError(err)
    //     })?;

    //     let maybe_marshalled_verification_code = conn
    //         .get::<String, Option<String>>(Self::verification_code_key(session))
    //         .await
    //         .map_err(|err| {
    //             tracing::error!(
    //                 ?err,
    //                 ?session,
    //                 ?email,
    //                 "Failed to get verification code in Redis"
    //             );
    //             AuthErrorCode::RedisConnectionError(err)
    //         })?;

    //     if let Some(marshalled_verification_code) =
    // maybe_marshalled_verification_code {         let verification_code =
    // VerificationCode::unmarshall(marshalled_verification_code);

    //         if verification_code.email == email
    //             && verification_code.code == *code
    //             && verification_code.session == *session
    //         {
    //             return Ok(true);
    //         }
    //     }

    //     tracing::warn!(?session, ?email, "Verification code not found in
    // storge");     Ok(false)
    // }

    // pub async fn drop_verification_code(&self, email: &str, session: &Uuid) ->
    // Result<bool> {     let mut conn =
    // self.redis.get_async_connection().await.map_err(|err| {
    //         tracing::error!(?err, "Failed to get Redis connection");
    //         AuthErrorCode::RedisConnectionError(err)
    //     })?;

    //     conn.del(Self::verification_code_key(session))
    //         .await
    //         .map_err(|err| {
    //             tracing::error!(
    //                 ?err,
    //                 ?session,
    //                 ?email,
    //                 "Failed to delete verification code in Redis"
    //             );
    //             AuthErrorCode::RedisConnectionError(err)
    //         })?;

    //     Ok(true)
    // }

    // fn verification_code_key(session: &Uuid) -> String {
    //     format!("{}{}", REDIS_VERIFICATION_CODE_PREFIX, session)
    // }
}
