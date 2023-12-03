use matrix::admin::resources::token::shared_secret::{
    SharedSecretRegistration, SharedSecretRegistrationDto,
};

use crate::tools::environment::Environment;

#[tokio::test]
async fn creates_user_using_shared_secret() {
    let env = Environment::new().await;
    let nonce = SharedSecretRegistration::get_nonce(&env.client)
        .await
        .unwrap()
        .nonce;
    let mac = SharedSecretRegistration::generate_mac(
        env.config.synapse_registration_shared_secret.clone(),
        nonce.clone(),
        "steve".into(),
        "verysecure".into(),
        true,
        None,
    )
    .unwrap();
    let registration = SharedSecretRegistration::create(
        &env.client,
        SharedSecretRegistrationDto {
            nonce,
            username: "steve".into(),
            displayname: Some("steve".into()),
            password: "verysecure".into(),
            admin: true,
            mac,
        },
    )
    .await
    .unwrap();

    assert!(!registration.access_token.is_empty());
    assert!(!registration.user_id.is_empty());
}
