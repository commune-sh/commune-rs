use commune::{
    account::{model::Account, service::CreateUnverifiedAccountDto},
    room::model::Room,
};
use fake::{
    faker,
    faker::internet::en::{Password, SafeEmail, Username},
    Fake,
};
use matrix::{
    admin::resources::room::{
        DeleteParams, ListParams, ListResponse, RoomService as AdminRoomService,
    },
    Client,
};

use commune::{room::service::CreateRoomDto, util::secret::Secret};

use crate::tools::environment::Environment;

struct AccountWithRoom {
    account: Account,
    _access_token: String,
    _room: Room,
}

async fn create_rooms(env: &Environment, i: usize) -> Vec<AccountWithRoom> {
    let mut result = Vec::with_capacity(i);

    for j in 0..i {
        let account_dto = CreateUnverifiedAccountDto {
            username: Username().fake::<String>().chars().take(12).collect(),
            password: Secret::new(Password(10..20).fake::<String>()),
            email: SafeEmail().fake::<String>(),
        };

        let room_dto = CreateRoomDto {
            name: format!("{j} - {username}'s room", username = account_dto.username),
            topic: format!(
                "{j} - discussion about {buzzword}",
                buzzword = faker::company::en::Buzzword().fake::<String>()
            ),
            alias: format!("{j}-{username}",  username = account_dto.username),
        };

        let account = env
            .commune
            .account
            .register_unverified(account_dto)
            .await
            .unwrap();
        let access_token = env
            .commune
            .account
            .issue_user_token(account.user_id.clone())
            .await
            .unwrap();
        let room = env
            .commune
            .room
            .create_public_room(&Secret::new(access_token.clone()), room_dto)
            .await
            .unwrap();

        result.push(AccountWithRoom {
            account,
            access_token,
            room,
        })
    }

    result
}

async fn remove_rooms(client: &Client) {
    let ListResponse { rooms, .. } = AdminRoomService::get_all(&client, ListParams::default())
        .await
        .unwrap();

    for room in rooms {
        AdminRoomService::delete_room(
            &client,
            room.room_id.as_ref(),
            DeleteParams {
                new_room: None,
                block: true,
                purge: true,
            },
        )
        .await
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use matrix::admin::resources::room::OrderBy;

    use super::*;

    #[tokio::test]
    async fn list_room() {
        let mut env = Environment::new().await;
        env.client.set_token(env.config.synapse_admin_token.clone()).unwrap();

        remove_rooms(&env.client).await;
        let accounts_with_room = create_rooms(&env, 10).await;

        let ListResponse { rooms, .. } = AdminRoomService::get_all(
            &env.client,
            ListParams {
                order_by: Some(OrderBy::Name),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert_eq!(
            rooms.iter().map(|r| r.name.clone().unwrap()).collect::<Vec<_>>(),
            accounts_with_room
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{} - {}'s room", i, v.account.display_name))
                .collect::<Vec<_>>()
        );
    }
}
