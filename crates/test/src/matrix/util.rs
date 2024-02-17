use anyhow::Result;
use futures::{future, TryFutureExt};
use matrix::{
    admin::resources::{
        room::{DeleteQuery, ListRoomQuery, ListRoomResponse, RoomService as AdminRoomService},
        user::{CreateUserBody, UserService as AdminUserService},
    },
    client::resources::{
        login::Login,
        room::{
            CreateRoomBody, JoinRoomBody, JoinRoomResponse, RoomPreset, RoomService, RoomVisibility,
        },
    },
    ruma_common::{OwnedRoomId, OwnedUserId, RoomId},
    Client,
};

use rand::Rng;

use crate::tools::environment::Environment;

pub struct Test {
    pub samples: Vec<Sample>,
    pub server_name: String,
    pub admin: Client,
}

pub struct Sample {
    pub user_ids: Vec<OwnedUserId>,
    pub room_id: OwnedRoomId,
    pub access_tokens: Vec<String>,
}

impl Sample {
    pub fn guests(&self) -> impl Iterator<Item = (&OwnedUserId, &String)> {
        self.user_ids.iter().zip(self.access_tokens.iter()).skip(1)
    }
    pub fn owner(&self) -> (&OwnedUserId, &String) {
        self.user_ids
            .iter()
            .zip(self.access_tokens.iter())
            .clone()
            .next()
            .unwrap()
    }
}

async fn create_accounts(
    client: &Client,
    server_name: String,
    amount: usize,
    room: usize,
    seed: u64,
) -> Vec<(OwnedUserId, String)> {
    let users: Vec<_> = (0..amount)
        .map(|i| OwnedUserId::try_from(format!("@{seed}-{room}-{i}:{}", server_name)).unwrap())
        .collect();

    future::try_join_all((0..amount).map(|i| {
        AdminUserService::create(
            &client,
            &users.get(i).unwrap(),
            CreateUserBody {
                password: "verysecure".to_owned(),
                logout_devices: false,
                displayname: None,
                avatar_url: None,
                threepids: vec![],
                external_ids: vec![],
                admin: false,
                deactivated: false,
                user_type: None,
                locked: false,
            },
        )
        .and_then(|resp| {
            Login::login_credentials(client, resp.name, "verysecure".to_owned())
                .map_ok(|resp| resp.access_token)
        })
    }))
    .await
    .map(|r| users.into_iter().zip(r).collect())
    .unwrap()
}

async fn create_rooms(client: &Client, seed: u64, tokens: &[String]) -> Vec<OwnedRoomId> {
    future::try_join_all((0..tokens.len()).map(|i| {
        let access_token = &tokens[i];

        RoomService::create(
            client,
            access_token.to_owned(),
            CreateRoomBody {
                name: format!("{seed}-{i}-room"),
                topic: format!("{seed}-{i}-room-topic"),
                room_alias_name: format!("{seed}-{i}-room-alias"),
                preset: Some(RoomPreset::PublicChat),
                visibility: Some(RoomVisibility::Public),
                ..Default::default()
            },
        )
        .map_ok(|resp| resp.room_id)
    }))
    .await
    .unwrap()
}

async fn remove_rooms(client: &Client) {
    let ListRoomResponse { rooms, .. } =
        AdminRoomService::get_all(client, ListRoomQuery::default())
            .await
            .unwrap();

    tracing::info!("purging all rooms!");

    future::try_join_all(rooms.iter().map(|room| {
        AdminRoomService::delete_room(
            client,
            room.room_id.as_ref(),
            DeleteQuery {
                new_room: None,
                block: true,
                purge: true,
            },
        )
    }))
    .await
    .unwrap();
}

pub async fn init() -> Test {
    let _ = tracing_subscriber::fmt::try_init();

    // set this higher or equal to the number of tests
    let rooms = 8;

    let users_per_room = 4;

    let seed = rand::thread_rng().gen();

    let env = Environment::new().await;

    let server_name = env.config.synapse_server_name.clone();
    let admin_token = env.config.synapse_admin_token.clone();
    let mut admin = env.client.clone();

    admin.set_token(admin_token).unwrap();
    remove_rooms(&admin).await;

    let accounts = future::join_all(
        (0..rooms)
            .map(|room| create_accounts(&admin, server_name.clone(), users_per_room, room, seed)),
    )
    .await;

    let rooms = create_rooms(
        &admin,
        seed,
        &accounts
            .iter()
            // make first user in the array the admin
            .map(|users| users[0].1.clone())
            .collect::<Vec<_>>(),
    )
    .await;

    let samples = accounts
        .into_iter()
        .zip(rooms.into_iter())
        .map(|(users, room_id)| (users.into_iter().unzip(), room_id))
        .map(|((user_ids, access_tokens), room_id)| Sample {
            user_ids,
            room_id,
            access_tokens,
        })
        .collect();

    Test {
        samples,
        server_name,
        admin,
    }
}

pub async fn join_helper(
    client: &Client,
    users: impl Iterator<Item = (&OwnedUserId, &String)>,
    room_id: &RoomId,
) -> Vec<Result<JoinRoomResponse>> {
    future::join_all(users.map(|(_, access_token)| {
        RoomService::join(
            &client,
            access_token.clone(),
            room_id.into(),
            JoinRoomBody::default(),
        )
    }))
    .await
}
