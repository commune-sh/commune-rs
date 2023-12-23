use std::sync::Arc;

use tracing::instrument;
use url::Url;
use validator::{Validate, ValidationError};

use matrix::admin::resources::user::{
    ListUsersParams, ThreePid, User as MatrixUser, UserCreateDto,
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
    pub session: String,
}

#[derive(Debug, Validate)]
pub struct CreateAccountDto {
    #[validate(custom = "CreateAccountDto::validate_username")]
    pub username: String,
    #[validate(custom = "CreateAccountDto::validate_password")]
    pub password: Secret,
    #[validate(email)]
    pub email: String,
    pub session: String,
    pub code: String,
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

    /// Returs true if the given email address is already registered
    pub async fn email_exists(&self, email: &str) -> Result<bool> {
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

        Ok(!exists.users.is_empty())
    }

    #[instrument(skip(self, dto))]
    pub async fn send_code(&self, dto: SendCodeDto) -> Result<()> {
        let code = self.auth.create_verification_code(dto.session).await?;

        self.mail
            .send_mail(
                "onboarding@commune.sh".into(),
                dto.email,
                "Welcome to Commune!".into(),
                EmailTemplate::VerificationCode {
                    name: String::from("John"),
                    code,
                },
            )
            .await?;

        Ok(())
    }

    #[instrument(skip(self, dto))]
    pub async fn register(&self, dto: CreateAccountDto) -> Result<Account> {
        dto.validate().map_err(|err| {
            tracing::warn!(?err, "Failed to validate user creation dto");
            AccountErrorCode::from(err)
        })?;

        if self.email_exists(&dto.email).await? {
            return Err(AccountErrorCode::UsernameTaken(dto.username).into());
        }

        let user_id = UserId::new(dto.username.clone(), self.admin.server_name().to_string());
        let avatar_url = Url::parse(DEFAULT_AVATAR_URL).map_err(|err| {
            tracing::error!(?err, "Failed to parse default avatar url");
            Error::Unknown
        })?;

        let matrix_user = MatrixUser::create(
            &self.admin,
            user_id,
            UserCreateDto {
                displayname: Some(dto.username),
                password: dto.password.to_string(),
                logout_devices: false,
                avatar_url: Some(avatar_url),
                threepids: vec![ThreePid {
                    medium: "email".to_string(),
                    address: dto.email,
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

        Ok(Account {
            username: displayname,
            email: threepid.address.to_owned(),
            session: dto.session,
            code: dto.code,
        })
    }
}

#[cfg(test)]
mod test {
    use validator::Validate;

    use crate::util::secret::Secret;

    use super::CreateAccountDto;

    #[test]
    fn ensure_username_is_not_too_short() {
        let dto = CreateAccountDto {
            username: "ab".to_string(),
            password: Secret::new("password"),
            email: "aby@mail.com".to_string(),
            code: "1234".to_string(),
            session: "synapse".to_string(),
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
            code: "1234".to_string(),
            session: "synapse".to_string(),
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
            code: "1234".to_string(),
            session: "synapse".to_string(),
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
            code: "1234".to_string(),
            session: "synapse".to_string(),
        };
        let err = dto.validate().err().unwrap();

        assert_eq!(err.to_string(), "username cannot contain uppercase letters");
    }
}
