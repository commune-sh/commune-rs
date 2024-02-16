use std::str::FromStr;

use matrix::{
    admin::resources::{
        room::{RoomService as AdminRoomService, ListRoomResponse, ListRoomQuery, DeleteQuery},
        user::{
            UserService as AdminUserService, CreateUserBody,
        },
        user_id::UserId,
    },
    client::resources::{room::{CreateRoomBody, RoomService, RoomPreset, JoinRoomResponse, JoinRoomBody, RoomVisibility}, login::{Login, LoginCredentials}},
    ruma_common::{OwnedRoomId, OwnedUserId, OwnedRoomOrAliasId},
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
    pub user_id: OwnedUserId,
    pub room_id: OwnedRoomId,
    pub access_token: String,
}

async fn create_accounts(
    client: &Client,
    server_name: String,
    amount: usize,
    seed: u64,
) -> Vec<(OwnedUserId, String)> {
    let mut result = Vec::with_capacity(amount);

    for i in 0..amount {
        let username = format!("{seed}-{i}");
        let user_id = UserId::new(username.clone(), server_name.clone());
        let password = "verysecure".to_owned();

        let body = CreateUserBody {
            password,
            logout_devices: false,
            displayname: None,
            avatar_url: None,
            threepids: vec![],
            external_ids: vec![],
            admin: false,
            deactivated: false,
            user_type: None,
            locked: false,
        };
        AdminUserService::create(&client, user_id.clone(), body)
            .await
            .unwrap();

        let password = "verysecure".to_owned();
        let LoginCredentials { access_token } =
            Login::login_credentials(client, username, password)
                .await
                .unwrap();

        let user_id = OwnedUserId::from_str(&user_id.to_string()).unwrap();
        result.push((user_id, access_token));
    }

    result
}

async fn create_rooms(client: &Client, accounts: &[(OwnedUserId, String)]) -> Vec<OwnedRoomId> {
    let mut result = Vec::with_capacity(accounts.len());

    for (user_id, access_token) in accounts {
        let id = user_id.localpart();
        let preset = Some(RoomPreset::PublicChat);

        let name = format!("{id}-room-name");
        let topic = format!("{id}-room-topic");
        let room_alias_name = format!("{id}-room-alias");
        let visibility = Some(RoomVisibility::Public);

        let body = CreateRoomBody {
            name,
            topic,
            room_alias_name,
            preset,
            visibility,
            ..Default::default()
        };
        let resp = RoomService::create(client, access_token, body).await.unwrap();

        result.push(resp.room_id);
    }

    result
}

async fn remove_rooms(client: &Client) {
    let ListRoomResponse { rooms, .. } =
        AdminRoomService::get_all(client, ListRoomQuery::default())
            .await
            .unwrap();
    let room_names: Vec<_> = rooms
        .iter()
        .map(|r| r.name.clone().unwrap_or(r.room_id.to_string()))
        .collect();

    tracing::info!(?room_names, "purging all rooms!");

    for room in rooms {
        AdminRoomService::delete_room(
            client,
            room.room_id.as_ref(),
            DeleteQuery {
                new_room: None,
                block: true,
                purge: true,
            },
        )
        .await
        .unwrap();
    }
}

pub async fn init() -> Test {
    let _ = tracing_subscriber::fmt::try_init();

    let seed = rand::thread_rng().gen();

    let env = Environment::new().await;

    let server_name = env.config.synapse_server_name.clone();
    let admin_token = env.config.synapse_admin_token.clone();
    let mut admin = env.client.clone();

    admin.set_token(admin_token).unwrap();
    remove_rooms(&admin).await;

    let accounts = create_accounts(&admin, server_name.clone(), 4, seed).await;
    let rooms = create_rooms(
        &admin,
        accounts
            .iter()
            .map(|(user_id, access_token)| (user_id.clone(), access_token.clone()))
            .collect::<Vec<_>>()
            .as_slice(),
    )
    .await;

    let samples = accounts
        .into_iter()
        .zip(rooms.into_iter())
        .map(|((user_id, access_token), room_id)| Sample {
            user_id,
            room_id,
            access_token,
        })
        .collect();

    Test {
        samples,
        server_name,
        admin,
    }
}

pub async fn join_helper(client: &Client, samples: &[Sample]) -> Vec<(
    OwnedRoomId,
    Vec<OwnedUserId>,
    Vec<anyhow::Result<JoinRoomResponse>>,
)> {
    let mut result = Vec::with_capacity(samples.len());

    for sample in samples {
        let guests: Vec<_> = samples
            .iter()
            .filter(|g| g.user_id != sample.user_id)
            .collect();
        let mut resps = Vec::with_capacity(guests.len());

        for guest in guests.iter() {
            let resp = RoomService::join(
                &client,
                guest.access_token.clone(),
                &OwnedRoomOrAliasId::from(sample.room_id.clone()),
                JoinRoomBody::default(),
            )
            .await;

            resps.push(resp);
        }

        result.push((
            sample.room_id.clone(),
            guests.iter().map(|g| g.user_id.clone()).collect(),
            resps,
        ));
    }

    result
}

