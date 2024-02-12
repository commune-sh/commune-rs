use std::str::FromStr;

use commune::{account::service::CreateUnverifiedAccountDto, room::model::Room};
use matrix::{
    admin::resources::room::{DeleteBody, ListBody, ListResponse, RoomService as AdminRoomService},
    ruma_common::{OwnedRoomId, OwnedUserId},
    Client,
};

use commune::{room::service::CreateRoomDto, util::secret::Secret};

use crate::tools::environment::Environment;

struct Sample {
    id: String,
    user_id: OwnedUserId,
    room_id: OwnedRoomId,
    // access_token: String,
}

async fn create_accounts(
    env: &Environment,
    amount: usize,
    seed: u64,
) -> Vec<(String, (OwnedUserId, String))> {
    let mut result = Vec::with_capacity(amount);

    for i in 0..amount {
        let id = format!("{seed}-{i}");

        let account_dto = CreateUnverifiedAccountDto {
            username: format!("{id}-username"),
            password: Secret::new("verysecure".to_owned()),
            email: format!("{id}-email@matrix.org"),
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

        let user_id = OwnedUserId::from_str(&account.user_id.to_string()).unwrap();
        result.push((id, (user_id, access_token)));
    }

    result
}

async fn create_rooms(env: &Environment, accounts: &[(String, String)]) -> Vec<OwnedRoomId> {
    let mut result = Vec::with_capacity(accounts.len());

    for (id, access_token) in accounts {
        let room_dto = CreateRoomDto {
            name: format!("{id}-room-name"),
            topic: format!("{id}-room-topic"),
            alias: format!("{id}-room-alias"),
        };
        dbg!(&room_dto);
        let Room { room_id } = env
            .commune
            .room
            .create_public_room(&Secret::new(access_token.clone()), room_dto)
            .await
            .unwrap();

        result.push(OwnedRoomId::from_str(&room_id).unwrap());
    }

    result
}

async fn remove_rooms(client: &Client) {
    let ListResponse { rooms, .. } = AdminRoomService::get_all(client, ListBody::default())
        .await
        .unwrap();

    for room in rooms {
        AdminRoomService::delete_room(
            client,
            room.room_id.as_ref(),
            DeleteBody {
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
    use std::sync::Once;

    use matrix::{
        admin::resources::room::{MessagesBody, OrderBy},
        ruma_common::{RoomId, ServerName},
        ruma_events::AnyStateEvent,
    };
    use rand::Rng;
    use tokio::sync::OnceCell;

    use super::*;

    pub struct Test {
        env: Environment,
        samples: Vec<Sample>,
        // seed: u64,
    }

    static TRACING: Once = Once::new();
    static TEST: OnceCell<Test> = OnceCell::const_new();

    async fn init() -> Test {
        TRACING.call_once(|| {
            tracing_subscriber::fmt::init();
        });

        let mut env = Environment::new().await;
        let seed = rand::thread_rng().gen();

        env.client
            .set_token(env.config.synapse_admin_token.clone())
            .unwrap();
        remove_rooms(&env.client).await;
        dbg!(&env.config.synapse_admin_token.clone());

        let accounts = create_accounts(&env, 4, seed).await;
        let rooms = create_rooms(
            &env,
            accounts
                .iter()
                .map(|(id, (_, access_token))| (id.clone(), access_token.clone()))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .await;
        let samples = accounts
            .into_iter()
            .zip(rooms.into_iter())
            .map(|((id, (user_id, _)), room_id)| Sample {
                id,
                user_id,
                room_id,
            })
            .collect();

        Test { env, samples }
    }

    #[tokio::test]
    async fn get_all_rooms() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;
        let host = env.config.synapse_server_name.as_str();

        let ListResponse { rooms: resp, .. } =
            AdminRoomService::get_all(&env.client, ListBody::default())
                .await
                .unwrap();

        assert_eq!(
            samples
                .iter()
                .map(|s| Some(format!("{id}-room-name", id = &s.id)))
                .collect::<Vec<_>>(),
            resp.iter().map(|r| r.name.clone()).collect::<Vec<_>>(),
        );
        assert_eq!(
            samples
                .iter()
                .map(|s| format!("#{id}-room-alias:{host}", id = &s.id))
                .collect::<Vec<_>>(),
            resp.iter()
                .map(|r| r.canonical_alias.clone().unwrap())
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            samples.iter().map(|s| &s.room_id).collect::<Vec<_>>(),
            resp.iter().map(|r| &r.room_id).collect::<Vec<_>>(),
        );

        let ListResponse { rooms: resp, .. } = AdminRoomService::get_all(
            &env.client,
            ListBody {
                order_by: OrderBy::Creator,
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert_eq!(
            samples
                .iter()
                .map(|s| s.user_id.to_string())
                .collect::<Vec<_>>(),
            resp.iter()
                .map(|r| r.creator.clone().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn get_all_rooms_err() {
        let Test { env, .. } = TEST.get_or_init(init).await;

        let _ = AdminRoomService::get_all(
            &env.client,
            ListBody {
                from: Some(u64::MAX),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn get_room_details() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;
        let host = env.config.synapse_server_name.as_str();

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_one(&env.client, &rand.room_id)
            .await
            .unwrap();

        assert_eq!(Some(format!("{}-room-name", rand.id)), resp.name);
        assert_eq!(
            Some(format!("#{}-room-alias:{host}", rand.id)),
            resp.canonical_alias,
        );
        assert_eq!(Some(rand.user_id.to_string()), resp.creator);
        assert_eq!(
            Some(format!("{}-room-topic", rand.id)),
            resp.details.and_then(|d| d.topic),
        );
        assert_eq!(resp.join_rules, Some("public".into()));

        assert!(!resp.public);
        assert!(resp.room_type.is_none());
    }

    #[tokio::test]
    #[should_panic]
    async fn get_room_details_err() {
        let Test { env, .. } = TEST.get_or_init(init).await;
        let host = env.config.synapse_server_name.as_str();

        let _ =
            AdminRoomService::get_one(&env.client, &RoomId::new(&ServerName::parse(host).unwrap()))
                .await
                .unwrap();
    }

    #[tokio::test]
    async fn get_room_events() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_room_events::<AnyStateEvent>(
            &env.client,
            &rand.room_id,
            // no idea what the type is
            MessagesBody {
                from: "".into(),
                to: Default::default(),
                limit: Default::default(),
                filter: Default::default(),
                direction: Default::default(),
            },
        )
        .await
        .unwrap();

        let events = resp.chunk.deserialize().unwrap();
        assert!(events.len() == 8);
    }

    #[tokio::test]
    #[should_panic]
    async fn get_room_events_err() {
        let Test { env, .. } = TEST.get_or_init(init).await;
        let host = env.config.synapse_server_name.as_str();

        let _ = AdminRoomService::get_room_events::<AnyStateEvent>(
            &env.client,
            <&RoomId>::try_from(host).unwrap(),
            MessagesBody {
                from: "".into(),
                to: Default::default(),
                limit: Default::default(),
                filter: Default::default(),
                direction: Default::default(),
            },
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn get_state_events() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_state(&env.client, &rand.room_id)
            .await
            .unwrap();

        assert!(resp
            .state
            .into_iter()
            .all(|state| state.kind.contains("room")));
    }

    #[tokio::test]
    #[should_panic]
    async fn get_state_events_err() {
        let Test { env, .. } = TEST.get_or_init(init).await;
        let host = env.config.synapse_server_name.as_str();

        let _ = AdminRoomService::get_state(&env.client, <&RoomId>::try_from(host).unwrap())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_members() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_members(&env.client, &rand.room_id)
            .await
            .unwrap();

        assert_eq!(resp.members, vec![rand.user_id.to_string()]);
    }

    #[tokio::test]
    #[should_panic]
    async fn get_members_err() {
        let Test { env, .. } = TEST.get_or_init(init).await;
        let host = env.config.synapse_server_name.as_str();

        let _ = AdminRoomService::get_members(&env.client, <&RoomId>::try_from(host).unwrap())
            .await
            .unwrap();
    }
}
