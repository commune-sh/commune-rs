use std::time::{SystemTime, UNIX_EPOCH};

use email_address::EmailAddress;
use matrix::admin::registration_tokens::new::*;
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{commune, error::Result};

pub async fn service(address: EmailAddress) -> Result<()> {
    let uni = Uniform::new('0', '9');
    let token: String = uni.sample_iter(rand::thread_rng()).take(6).collect();

    let req = Request::new(
        token.clone(),
        1,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            // panics below should never happen
            .expect("system time overflow")
            .as_millis()
            .try_into()
            .expect("system time overflow"),
    );

    commune()
        .send_matrix_request(req, Some(&commune().config.matrix.admin_token.inner()))
        .await?;

    commune().send_email_verification(address, token).await?;

    Ok(())
}
