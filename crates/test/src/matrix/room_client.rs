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
        client::resources::room::{
            ForgetRoomBody, JoinRoomBody, JoinRoomResponse, LeaveRoomBody, Room as RoomService,
            RoomKickOrBanBody,
        },
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

    async fn join_helper() -> Vec<(
        OwnedRoomId,
        Vec<OwnedUserId>,
        Vec<anyhow::Result<JoinRoomResponse>>,
    )> {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let mut result = Vec::with_capacity(samples.len());

        for sample in samples {
            let client = env.client.clone();

            let guests: Vec<_> = samples.iter().filter(|g| g.id != sample.id).collect();
            let mut resps = Vec::with_capacity(guests.len());

            for guest in guests.iter() {
                let client = client.clone();

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

    #[tokio::test]
    async fn join_all_rooms() {
        let Test { env, .. } = TEST.get_or_init(init).await;

        let mut admin = env.client.clone();
        admin.set_token(&env.config.synapse_admin_token).unwrap();

        // first join
        let result = join_helper().await;
        let rooms: Vec<_> = result.iter().map(|r| &r.0).collect();
        tracing::info!(?rooms, "joining all guests");

        // check whether all guests are in the room and joined the expected room
        for (room_id, guests, resps) in result {
            let mut resp = AdminRoomService::get_members(&admin, &room_id)
                .await
                .unwrap();
            resp.members.sort();

            assert!(resps.iter().all(|r| r.is_ok()));
            let resps: Vec<_> = resps.into_iter().flatten().collect();

            assert!(resps.iter().all(|r| r.room_id == *room_id));

            for guest in guests {
                assert!(resp.members.contains(&guest));
            }
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

        // check whether all guests left the room
        for room_id in result {
            let resp = AdminRoomService::get_members(&admin, &room_id)
                .await
                .unwrap();

            assert_eq!(resp.members.len(), 1);
            assert_eq!(
                &[samples
                    .iter()
                    .find(|s| s.room_id == room_id)
                    .map(|s| s.user_id.clone())
                    .unwrap()],
                resp.members.as_slice()
            );
        }
    }

    #[tokio::test]
    async fn forget_all_rooms() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let mut admin = env.client.clone();
        admin.set_token(&env.config.synapse_admin_token).unwrap();

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
        }

        // check whether all guests are still not present anymore the room
        for sample in samples {
            let room_id = &sample.room_id;

            let resp = AdminRoomService::get_members(&admin, room_id)
                .await
                .unwrap();

            assert_eq!(resp.members.len(), 1);
            assert_eq!(
                &[samples
                    .iter()
                    .find(|s| &s.room_id == room_id)
                    .map(|s| s.user_id.clone())
                    .unwrap()],
                resp.members.as_slice()
            );
        }

        // confirm a room can't be forgotten if we didn't leave first
        for sample in samples {
            let client = env.client.clone();
            let room_id = &sample.room_id;

            let resp = RoomService::forget(
                &client,
                sample.access_token.clone(),
                room_id,
                ForgetRoomBody::default(),
            )
            .await;

            assert!(resp.is_err());
        }
    }

    #[tokio::test]
    async fn kick_all_guests() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let mut admin = env.client.clone();
        admin.set_token(&env.config.synapse_admin_token).unwrap();

        // second join
        let result = join_helper().await;
        let rooms: Vec<_> = result.iter().map(|r| &r.0).collect();
        tracing::info!(?rooms, "joining all guests");

        // check whether all guests are in the room and joined the expected room
        for (room_id, guests, resps) in result.iter() {
            let mut resp = AdminRoomService::get_members(&admin, room_id)
                .await
                .unwrap();
            resp.members.sort();

            assert!(resps.iter().all(|r| r.is_ok()));
            let resps: Vec<_> = resps.iter().flatten().collect();

            assert!(resps.iter().all(|r| r.room_id == *room_id));

            for guest in guests {
                assert!(resp.members.contains(guest));
            }
        }

        for sample in samples {
            let client = env.client.clone();
            let room_id = &sample.room_id;

            let guests: Vec<_> = samples.iter().filter(|g| g.id != sample.id).collect();

            for guest in guests {
                let client = client.clone();

                RoomService::kick(
                    &client,
                    guest.access_token.clone(),
                    room_id,
                    RoomKickOrBanBody {
                        reason: Default::default(),
                        user_id: guest.user_id.clone(),
                    },
                )
                .await
                .unwrap();
            }
        }

        // check whether all guests left the room
        for (room_id, _, _) in result {
            let resp = AdminRoomService::get_members(&admin, &room_id)
                .await
                .unwrap();

            assert_eq!(resp.members.len(), 1);
            assert_eq!(
                &[samples
                    .iter()
                    .find(|s| s.room_id == room_id)
                    .map(|s| s.user_id.clone())
                    .unwrap()],
                resp.members.as_slice()
            );
        }
    }

    #[tokio::test]
    async fn ban_all_guests() {
        let Test { env, samples, .. } = TEST.get_or_init(init).await;

        let mut admin = env.client.clone();
        admin.set_token(&env.config.synapse_admin_token).unwrap();

        // third join
        let result = join_helper().await;
        let rooms: Vec<_> = result.iter().map(|r| &r.0).collect();
        tracing::info!(?rooms, "joining all guests");

        // check whether all guests are in the room and joined the expected room
        for (room_id, guests, resps) in result.iter() {
            let mut resp = AdminRoomService::get_members(&admin, room_id)
                .await
                .unwrap();
            resp.members.sort();

            assert!(resps.iter().all(|r| r.is_ok()));
            let resps: Vec<_> = resps.iter().flatten().collect();

            assert!(resps.iter().all(|r| r.room_id == *room_id));

            for guest in guests {
                assert!(resp.members.contains(guest));
            }
        }

        for sample in samples {
            let client = env.client.clone();

            let guests: Vec<_> = samples.iter().filter(|g| g.id != sample.id).collect();

            for guest in guests {
                let client = client.clone();
                let room_id = &sample.room_id;

                RoomService::ban(
                    &client,
                    sample.access_token.clone(),
                    room_id,
                    RoomKickOrBanBody {
                        reason: Default::default(),
                        user_id: guest.user_id.clone(),
                    },
                )
                .await
                .unwrap();
            }
        }

        // fourth join
        let result = join_helper().await;
        let rooms: Vec<_> = result.iter().map(|r| &r.0).collect();
        tracing::info!(?rooms, "joining all guests");

        // check whether all guests got banned from the room
        // check whether their join request failed
        for (room_id, _, resps) in result {
            let resp = AdminRoomService::get_members(&admin, &room_id)
                .await
                .unwrap();

            assert_eq!(resp.members.len(), 1);
            assert_eq!(
                &[samples
                    .iter()
                    .find(|s| s.room_id == room_id)
                    .map(|s| s.user_id.clone())
                    .unwrap()],
                resp.members.as_slice()
            );

            assert!(resps.iter().all(|r| r.is_err()));
        }

        for sample in samples {
            let client = env.client.clone();

            let guests: Vec<_> = samples.iter().filter(|g| g.id != sample.id).collect();

            for guest in guests {
                let client = client.clone();
                let room_id = &sample.room_id;

                RoomService::unban(
                    &client,
                    sample.access_token.clone(),
                    room_id,
                    RoomKickOrBanBody {
                        reason: Default::default(),
                        user_id: guest.user_id.clone(),
                    },
                )
                .await
                .unwrap();
            }
        }
    }
}
