//! [Shared-Secret Registration API](https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#)
//!
//! # Important
//!
//! This API is disabled when MSC3861 is enabled. See [#15582](https://github.com/matrix-org/synapse/pull/15582)
//!
//! This API allows for the creation of users in an administrative and
//! non-interactive way. This is generally used for bootstrapping a Synapse
//! instance with administrator accounts.
//!
//! To authenticate yourself to the server, you will need both the shared secret
//! (registration_shared_secret in the homeserver configuration), and a one-time
//! nonce. If the registration shared secret is not configured, this API is not
//! enabled.

use anyhow::Result;
use hex;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha1::Sha1;

use crate::admin::Client;

type HmacSha1 = Hmac<Sha1>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Nonce {
    pub nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedSecretRegistrationDto {
    pub nonce: String,
    pub username: String,
    pub displayname: Option<String>,
    pub password: String,
    pub admin: bool,
    /// The MAC is the hex digest output of the HMAC-SHA1 algorithm, with the
    /// key being the shared secret and the content being the nonce, user,
    /// password, either the string "admin" or "notadmin", and optionally the
    /// user_type each separated by NULs.
    pub mac: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedSecretRegistration {
    pub access_token: String,
    pub user_id: String,
    pub home_server: String,
    pub device_id: String,
}

impl SharedSecretRegistration {
    /// Fetches the `Nonce` from the server.
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#shared-secret-registration
    pub async fn get_nonce(client: &Client) -> Result<Nonce> {
        let resp = client.get("/_synapse/admin/v1/register").await?;

        Ok(resp.json().await?)
    }

    /// Creates the [`SharedSecretRegistration`] instance.
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#shared-secret-registration
    pub async fn create(client: &Client, dto: SharedSecretRegistrationDto) -> Result<Self> {
        let resp = client
            .post_json("/_synapse/admin/v1/register", &dto)
            .await?;

        Ok(resp.json().await?)
    }

    /// Generates the MAC.
    ///
    /// # Inspiration
    ///
    /// This implementation is inspired by the following Python code from the
    /// Synapse documentation on `Shared-Secret Registration`.
    ///
    /// ```python
    /// import hmac, hashlib
    ///
    /// def generate_mac(nonce, user, password, admin=False, user_type=None):
    ///
    ///     mac = hmac.new(
    ///       key=shared_secret,
    ///       digestmod=hashlib.sha1,
    ///     )
    ///
    ///     mac.update(nonce.encode('utf8'))
    ///     mac.update(b"\x00")
    ///     mac.update(user.encode('utf8'))
    ///     mac.update(b"\x00")
    ///     mac.update(password.encode('utf8'))
    ///     mac.update(b"\x00")
    ///     mac.update(b"admin" if admin else b"notadmin")
    ///     if user_type:
    ///         mac.update(b"\x00")
    ///         mac.update(user_type.encode('utf8'))
    ///
    ///     return mac.hexdigest()
    /// ```
    /// [Source](https://matrix-org.github.io/synapse/latest/admin_api/register_api.html#shared-secret-registration)
    pub fn generate_mac<S: AsRef<str>>(
        shared_secret: S,
        nonce: S,
        user: S,
        password: S,
        admin: bool,
        user_type: Option<S>,
    ) -> Result<String> {
        let mut mac = HmacSha1::new_from_slice(shared_secret.as_ref().as_bytes())?;

        mac.update(nonce.as_ref().as_bytes());
        mac.update(b"\x00");

        mac.update(user.as_ref().as_bytes());
        mac.update(b"\x00");

        mac.update(password.as_ref().as_bytes());
        mac.update(b"\x00");

        if admin {
            mac.update("admin".as_bytes());
        } else {
            mac.update("notadmin".as_bytes());
        }

        if let Some(user_type) = user_type {
            mac.update(b"\x00");
            mac.update(user_type.as_ref().as_bytes());
        }

        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        Ok(hex::encode(code_bytes))
    }
}

#[cfg(test)]
mod test {
    use super::SharedSecretRegistration;

    #[test]
    fn generates_mac_accordingly() {
        let want = "c272fb1c287c795ff5ce238c4dba57cf95db5eff";
        let have = SharedSecretRegistration::generate_mac(
            "m@;wYOUOh0f:CH5XA65sJB1^q01~DmIriOysRImot,OR_vzN&B",
            "1234567890",
            "groot",
            "imroot!1234",
            true,
            None,
        )
        .unwrap();

        assert_eq!(have, want);
    }
}
