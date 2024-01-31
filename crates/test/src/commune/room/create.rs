use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Fake;

use commune::account::service::CreateUnverifiedAccountDto;
use commune::auth::service::{LoginCredentials, LoginCredentialsResponse};
use commune::room::service::CreateRoomDto;
use commune::util::secret::Secret;

use crate::tools::environment::Environment;

#[tokio::test]
async fn creates_public_chat_room() {
    let env = Environment::new().await;
    let username: String = Username().fake();
    let password: String = Password(10..20).fake();
    let email: String = FreeEmail().fake();
    let password = Secret::new(password);

    env.commune
        .account
        .register_unverified(CreateUnverifiedAccountDto {
            username: username.clone(),
            password: password.clone(),
            email,
        })
        .await
        .expect("Failed to register account");

    let LoginCredentialsResponse { access_token } = env
        .commune
        .auth
        .login(LoginCredentials { username, password })
        .await
        .expect("Failed to login");

    let room_name = String::from("MyVeryFirstPublicRoom");
    let room_topic = String::from("MyVeryFirstPublicRoomTopic");
    let room_alias = String::from("MyVeryFirstPublicRoomAlias");
    let room = env
        .commune
        .room
        .create_public_room(
            &access_token,
            CreateRoomDto {
                name: room_name,
                topic: room_topic,
                alias: room_alias,
            },
        )
        .await
        .expect("Failed to create public room");

    assert!(!room.room_id.is_empty(), "should return room_id");
}
