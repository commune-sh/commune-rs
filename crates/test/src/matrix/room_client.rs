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
    access_token: String,
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
    let room_names: Vec<_> = rooms
        .iter()
        .map(|r| r.name.clone().unwrap_or(r.room_id.to_string()))
        .collect();

    tracing::info!(?room_names, "purging all rooms!");

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
        admin::resources::room::RoomService as AdminRoomService,
        client::resources::room::{JoinRoomBody, Room as RoomService, LeaveRoomBody, ForgetRoomBody},
        ruma_common::OwnedRoomOrAliasId,
    };
    use rand::Rng;
    use tokio::sync::OnceCell;

    use super::*;

    pub struct Test {
        env: Environment,
        samples: Vec<Sample>,
    }

    static TRACING: Once = Once::new();
    static TEST: OnceCell<Test> = OnceCell::const_new();

    async fn init() -> Test {
        TRACING.call_once(|| {
            let _ = tracing_subscriber::fmt::try_init();
        });

        let mut env = Environment::new().await;
        let seed = rand::thread_rng().gen();

        env.client
            .set_token(env.config.synapse_admin_token.clone())
            .unwrap();
        remove_rooms(&env.client).await;

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
            .map(|((id, (user_id, access_token)), room_id)| Sample {
                id,
                user_id,
                room_id,
                access_token,
            })
            .collect();

        Test { env, samples }
    }

    #[tokio::test]
    async fn join_all_rooms() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let mut admin = env.client.clone();
        admin.set_token(&env.config.synapse_admin_token).unwrap();

        let mut result = Vec::with_capacity(samples.len());

        for sample in samples {
            let client = env.client.clone();

            let guests: Vec<_> = samples.iter().filter(|g| g.id != sample.id).collect();
            let mut resps = Vec::with_capacity(guests.len());

            for guest in guests {
                let client = client.clone();

                let resp = RoomService::join(
                    &client,
                    guest.access_token.clone(),
                    &OwnedRoomOrAliasId::from(sample.room_id.clone()),
                    JoinRoomBody::default(),
                )
                .await
                .unwrap();

                resps.push(resp);
            }

            result.push((sample.room_id.clone(), resps));
        }

        for (room_id, resps) in result {
            let mut resp = AdminRoomService::get_members(&admin, &room_id)
                .await
                .unwrap();
            resp.members.sort();

            let guests: Vec<_> = samples
                .iter()
                .map(|g| g.user_id.to_string())
                .collect();

            assert!(resps.iter().all(|r| r.room_id.clone() == room_id));

            assert_eq!(guests, resp.members);
        }
    }

    #[tokio::test]
    async fn leave_all_rooms() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let mut admin = env.client.clone();
        admin.set_token(&env.config.synapse_admin_token).unwrap();

        let mut result = Vec::with_capacity(samples.len());

        for sample in samples {
            let client = env.client.clone();

            let guests: Vec<_> = samples.iter().filter(|g| g.id != sample.id).collect();

            for guest in guests {
                let client = client.clone();

                RoomService::leave(
                    &client,
                    guest.access_token.clone(),
                    &sample.room_id,
                    LeaveRoomBody::default(),
                )
                .await
                .unwrap();
            }

            result.push(sample.room_id.clone());
        }

        for room_id in result {
            let resp = AdminRoomService::get_members(&admin, &room_id)
                .await
                .unwrap();

            assert_eq!(resp.members.len(), 1);
            assert_eq!(&samples.iter().find(|s| s.room_id == room_id).map(|s| s.user_id.to_string()).unwrap(), resp.members.first().unwrap());
        }
    }

    #[tokio::test]
    async fn forget_all_rooms() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let mut admin = env.client.clone();
        admin.set_token(&env.config.synapse_admin_token).unwrap();

        let mut result = Vec::with_capacity(samples.len());

        for sample in samples {
            let client = env.client.clone();

            let guests: Vec<_> = samples.iter().filter(|g| g.id != sample.id).collect();

            for guest in guests {
                let client = client.clone();

                RoomService::forget(
                    &client,
                    guest.access_token.clone(),
                    &sample.room_id,
                    ForgetRoomBody::default(),
                )
                .await
                .unwrap();
            }

            result.push(sample.room_id.clone());
        }

        for room_id in result {
            let resp = AdminRoomService::get_members(&admin, &room_id)
                .await
                .unwrap();

            assert_eq!(resp.members.len(), 1);
            assert_eq!(&samples.iter().find(|s| s.room_id == room_id).map(|s| s.user_id.to_string()).unwrap(), resp.members.first().unwrap());
        }
    }
}
