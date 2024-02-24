use ruma_common::OwnedUserId;
use ruma_macros::request;




#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(query)]
    pub user_id: OwnedUserId,
}
