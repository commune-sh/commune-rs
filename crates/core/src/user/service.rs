use anyhow::Result;
use validator::Validate;

use matrix::admin::resources::user::{ThreePid, User as MatrixUser, UserCreateDto};
use matrix::admin::resources::user_id::UserId;
use matrix::admin::Client as MatrixAdminClient;

use crate::util::time::timestamp;

use super::model::User;

#[derive(Debug, Validate)]
pub struct CreateAccountDto {
    #[validate(length(min = 8, max = 12))]
    pub username: String,
    #[validate(length(min = 8, max = 12))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub session: String,
    pub code: String,
}

pub struct UserService {
    admin: MatrixAdminClient,
}

impl UserService {
    pub fn new(admin: MatrixAdminClient) -> Self {
        Self { admin }
    }

    pub async fn register(&self, dto: CreateAccountDto) -> Result<User> {
        dto.validate()?;

        let user_id = UserId::new(dto.username.clone(), self.admin.server_name().to_string());
        let matrix_user = MatrixUser::create(
            &self.admin,
            user_id,
            UserCreateDto {
                displayname: Some(dto.username),
                password: dto.password,
                logout_devices: false,
                avatar_url: None,
                threepids: vec![ThreePid {
                    medium: "email".to_string(),
                    address: dto.email,
                    added_at: timestamp().unwrap(),
                    validated_at: timestamp().unwrap(),
                }],
                external_ids: Vec::default(),
                admin: false,
                deactivated: false,
                user_type: None,
                locked: false,
            },
        )
        .await
        .unwrap();

        Ok(User {
            username: matrix_user.displayname.unwrap(),
            email: matrix_user.threepids.first().unwrap().address.clone(),
            session: dto.session,
            code: dto.code,
        })
    }
}
