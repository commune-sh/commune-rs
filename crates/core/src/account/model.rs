use url::Url;

use matrix::admin::resources::user_id::UserId;

#[derive(Debug, Clone)]
pub struct Account {
    pub user_id: UserId,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<Url>,
    pub age: i64,
    pub admin: bool,
    pub verified: bool,
}
