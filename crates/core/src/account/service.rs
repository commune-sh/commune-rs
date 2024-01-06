use std::sync::Arc;

use tracing::instrument;
use url::Url;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use matrix::admin::resources::user::{
    ListUsersParams, LoginAsUserDto, ThreePid, User as MatrixUser, UserCreateDto,
};
use matrix::admin::resources::user_id::UserId;
use matrix::Client as MatrixAdminClient;

use crate::auth::service::AuthService;
use crate::mail::service::{EmailTemplate, MailService};
use crate::util::secret::Secret;
use crate::util::time::timestamp;
use crate::{Error, Result};

use super::error::AccountErrorCode;
use super::model::Account;

const DEFAULT_AVATAR_URL: &str = "https://via.placeholder.com/150";
const MIN_USERNAME_LENGTH: usize = 3;
const MAX_USERNAME_LENGTH: usize = 12;
const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug, Validate)]
pub struct SendCodeDto {
    #[validate(email)]
    pub email: String,
    pub session: Uuid,
}

#[derive(Debug, Validate)]
pub struct VerifyCodeDto {
    #[validate(email)]
    pub email: String,
    pub session: Uuid,
    pub code: Secret,
}

#[derive(Debug, Validate)]
pub struct CreateAccountDto {
    #[validate(custom = "CreateAccountDto::validate_username")]
    pub username: String,
    #[validate(custom = "CreateAccountDto::validate_password")]
    pub password: Secret,
    #[validate(email)]
    pub email: String,
    pub session: Uuid,
    pub code: Secret,
}

impl CreateAccountDto {
    /// Validation logic for usernames enforced in user creation
    fn validate_username(username: &str) -> std::result::Result<(), ValidationError> {
        if username.len() < MIN_USERNAME_LENGTH {
            return Err(ValidationError::new("username is too short"));
        }

        if username.len() > MAX_USERNAME_LENGTH {
            return Err(ValidationError::new("username is too long"));
        }

        if username.contains(' ') {
            return Err(ValidationError::new("username cannot contain spaces"));
        }

        if username.to_ascii_lowercase() != username {
            return Err(ValidationError::new(
                "username cannot contain uppercase letters",
            ));
        }

        Ok(())
    }

    /// Validation logic for passwords enforced in user creation
    fn validate_password(password: &Secret) -> std::result::Result<(), ValidationError> {
        if password.inner().len() < MIN_PASSWORD_LENGTH {
            return Err(ValidationError::new("password is too short"));
        }

        Ok(())
    }
}

pub struct AccountService {
    admin: Arc<MatrixAdminClient>,
    auth: Arc<AuthService>,
    mail: Arc<MailService>,
}

impl AccountService {
    pub fn new(
        admin: Arc<MatrixAdminClient>,
        auth: Arc<AuthService>,
        mail: Arc<MailService>,
    ) -> Self {
        Self { admin, auth, mail }
    }

    /// Returs `true` if the given `email address` is NOT registered in the
    /// Matrix Server
    pub async fn is_email_available(&self, email: &str) -> Result<bool> {
        let user_id = UserId::new(email, self.admin.server_name());
        let exists = MatrixUser::list(
            &self.admin,
            ListUsersParams {
                user_id: Some(user_id.to_string()),
                ..Default::default()
            },
        )
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to list users");
            Error::Unknown
        })?;

        Ok(exists.users.is_empty())
    }

    /// Sends a verification code to the given email address
    #[instrument(skip(self, dto))]
    pub async fn send_code(&self, dto: SendCodeDto) -> Result<()> {
        let verification_code = self
            .auth
            .send_verification_code(&dto.email, &dto.session)
            .await?;

        self.mail
            .send_mail(
                String::from("onboarding@commune.sh"),
                dto.email,
                EmailTemplate::VerificationCode {
                    code: verification_code.code,
                },
            )
            .await?;

        Ok(())
    }

    /// Verifies the given verification code against the given email address
    /// and session id
    #[instrument(skip(self, dto))]
    pub async fn verify_code(&self, dto: VerifyCodeDto) -> Result<bool> {
        let result = self
            .auth
            .check_verification_code(&dto.email, &dto.session, &dto.code)
            .await?;

        Ok(result)
    }

    /// Registers a new user account in Matrix Server
    #[instrument(skip(self, dto))]
    pub async fn register(&self, dto: CreateAccountDto) -> Result<Account> {
        if !self
            .auth
            .check_verification_code(&dto.email, &dto.session, &dto.code)
            .await?
        {
            return Err(AccountErrorCode::InvalidVerificationCode.into());
        }

        dto.validate().map_err(|err| {
            tracing::warn!(?err, "Failed to validate user creation dto");
            AccountErrorCode::from(err)
        })?;

        if !self.is_email_available(&dto.email).await? {
            return Err(AccountErrorCode::EmailTaken(dto.email).into());
        }

        let user_id = UserId::new(dto.username.clone(), self.admin.server_name().to_string());
        let avatar_url = Url::parse(DEFAULT_AVATAR_URL).map_err(|err| {
            tracing::error!(?err, "Failed to parse default avatar url");
            Error::Unknown
        })?;

        let matrix_user = MatrixUser::create(
            &self.admin,
            user_id.clone(),
            UserCreateDto {
                displayname: Some(dto.username),
                password: dto.password.to_string(),
                logout_devices: false,
                avatar_url: Some(avatar_url),
                threepids: vec![ThreePid {
                    medium: "email".to_string(),
                    address: dto.email.clone(),
                    added_at: timestamp()?,
                    validated_at: timestamp()?,
                }],
                external_ids: Vec::default(),
                admin: false,
                deactivated: false,
                user_type: None,
                locked: false,
            },
        )
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to create user");
            Error::Unknown
        })?;

        let Some(displayname) = matrix_user.displayname else {
            tracing::error!("Failed to get displayname for user");
            return Err(Error::Unknown);
        };

        let Some(threepid) = matrix_user.threepids.first() else {
            tracing::error!("Failed to get threepid for user");
            return Err(Error::Unknown);
        };

        self.auth
            .drop_verification_code(&dto.email, &dto.session)
            .await?;

        Ok(Account {
            user_id,
            username: displayname.clone(),
            email: threepid.address.to_owned(),
            display_name: displayname,
            avatar_url: matrix_user.avatar_url,
            age: 0,
            admin: matrix_user.admin,
            verified: true,
        })
    }

    /// Creates an access token for the given user
    pub async fn issue_user_token(&self, user_id: UserId) -> Result<String> {
        let credentials =
            MatrixUser::login_as_user(&self.admin, user_id.clone(), LoginAsUserDto::default())
                .await
                .map_err(|err| {
                    tracing::error!(?err, ?user_id, "Failed to login as user");
                    Error::Unknown
                })?;

        Ok(credentials.access_token)
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;
    use validator::Validate;

    use crate::util::secret::Secret;

    use super::CreateAccountDto;

    #[test]
    fn ensure_username_is_not_too_short() {
        let dto = CreateAccountDto {
            username: "ab".to_string(),
            password: Secret::new("password"),
            email: "aby@mail.com".to_string(),
            code: Secret::new("1234"),
            session: Uuid::new_v4(),
        };
        let err = dto.validate().err().unwrap();

        assert_eq!(err.to_string(), "username is too short");
    }

    #[test]
    fn ensure_username_is_not_too_long() {
        let dto = CreateAccountDto {
            username: "abbeyroadismyfavoritealbum".to_string(),
            password: Secret::new("password"),
            email: "aby@mail.com".to_string(),
            code: Secret::new("1234"),
            session: Uuid::new_v4(),
        };
        let err = dto.validate().err().unwrap();

        assert_eq!(err.to_string(), "username is too long");
    }

    #[test]
    fn ensure_username_does_not_contain_spaces() {
        let dto = CreateAccountDto {
            username: "abbey road".to_string(),
            password: Secret::new("password"),
            email: "aby@mail.com".to_string(),
            code: Secret::new("1234"),
            session: Uuid::new_v4(),
        };
        let err = dto.validate().err().unwrap();

        assert_eq!(err.to_string(), "username cannot contain spaces");
    }

    #[test]
    fn ensure_username_is_lowercased() {
        let dto = CreateAccountDto {
            username: "AbbeyRoad".to_string(),
            password: Secret::new("password"),
            email: "aby@mail.com".to_string(),
            code: Secret::new("1234"),
            session: Uuid::new_v4(),
        };
        let err = dto.validate().err().unwrap();

        assert_eq!(err.to_string(), "username cannot contain uppercase letters");
    }
}
