use matrix::admin::resources::user_id::UserId;

#[derive(Debug, Clone)]
pub struct Account {
    pub user_id: UserId,
    pub username: String,
    pub email: String,
}
