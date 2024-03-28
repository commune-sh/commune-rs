use std::time::UNIX_EPOCH;

use email_address::EmailAddress;
use matrix::admin::registration_tokens::new::*;
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{commune, error::Result};

pub async fn service(address: EmailAddress) -> Result<String> {
    let uni = Uniform::new('0', '9');
    let token: String = uni.sample_iter(rand::thread_rng()).take(6).collect();

    // TODO: decide on whether to use timezones or UTC
    let offset = UNIX_EPOCH.elapsed().unwrap().as_millis() + 60 * 60 * 60;
    let req = Request::new(token.clone(), 1, offset.try_into().unwrap());

    commune()
        .send_matrix_request(req, Some(&commune().config.matrix.admin_token.inner()))
        .await?;

    commune()
        .send_email_verification(address, token.clone())
        .await?;

    Ok(token)
}
