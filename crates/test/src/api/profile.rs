pub mod avatar_url;
pub mod displayname;

use matrix::client::profile::Response;
use url::form_urlencoded::byte_serialize;

use crate::env::Env;

pub async fn get_profile(
    client: &Env,
    login_resp: &matrix::client::login::Response,
) -> Result<Response, reqwest::Error> {
    let resp = client
        .get(&format!(
            "/_commune/client/r0/profile/{}",
            byte_serialize(login_resp.user_id.as_bytes()).collect::<String>(),
        ))
        .send()
        .await?;

    resp.json().await
}
