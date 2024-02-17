#[cfg(test)]
mod tests {
    use futures::{future, FutureExt};
    use matrix::{
        admin::resources::room::RoomService as AdminRoomService,
        client::resources::room::{ForgetRoomBody, LeaveRoomBody, RoomKickOrBanBody, RoomService},
    };
    use tokio::sync::OnceCell;

    use crate::matrix::util::{self, join_helper, Test};

    static TEST: OnceCell<Test> = OnceCell::const_new();

    #[tokio::test]
    async fn join_all_rooms() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let result = future::join_all(samples.iter().map(|s| {
            join_helper(&client, s.guests(), &s.room_id)
                .map(|resp| (&s.room_id, s.guests().map(|(id, _)| id), resp))
        }))
        .await;

        tracing::info!("joining all guests");

        // check whether all guests are in the room and joined the expected room
        for (room_id, guests, resps) in result {
            let mut resp = AdminRoomService::get_members(&admin, room_id)
                .await
                .unwrap();
            resp.members.sort();

            assert!(resps.iter().all(Result::is_ok));
            assert!(resps.iter().flatten().all(|r| &r.room_id == room_id));
            assert!(guests.cloned().all(|guest| resp.members.contains(&guest)));
        }
    }

    #[tokio::test]
    async fn leave_all_rooms() {
        let Test { samples, admin, .. } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        for sample in samples {
            for (_, access_token) in sample.guests() {
                RoomService::leave(
                    &client,
                    access_token,
                    &sample.room_id,
                    LeaveRoomBody::default(),
                )
                .await
                .unwrap();
            }
        }

        // check whether all guests left the room
        for sample in samples {
            let resp = AdminRoomService::get_members(&admin, &sample.room_id)
                .await
                .unwrap();

            assert_eq!(resp.members.len(), 1);
            assert_eq!(
                &[samples
                    .iter()
                    .find(|s| s.room_id == sample.room_id)
                    .map(|s| s.owner())
                    .map(|(id, _)| id.to_owned())
                    .unwrap()],
                resp.members.as_slice()
            );
        }
    }

    #[tokio::test]
    async fn forget_all_rooms() {
        let Test { samples, admin, .. } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        for sample in samples {
            for (_, access_token) in sample.guests() {
                RoomService::forget(
                    &client,
                    access_token,
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
                    .map(|s| s.owner())
                    .map(|(id, _)| id.to_owned())
                    .unwrap()],
                resp.members.as_slice()
            );
        }

        // confirm a room can't be forgotten if we didn't leave first
        for sample in samples {
            let room_id = &sample.room_id;
            let (_, access_token) = sample.owner();

            let resp =
                RoomService::forget(&client, access_token, room_id, ForgetRoomBody::default())
                    .await;

            assert!(resp.is_err());
        }
    }

    #[tokio::test]
    async fn kick_all_guests() {
        let Test { samples, admin, .. } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        // second join
        let result = future::join_all(samples.iter().map(|s| {
            join_helper(&client, s.guests(), &s.room_id)
                .map(|resp| (&s.room_id, s.guests().map(|(id, _)| id), resp))
        }))
        .await;

        tracing::info!("joining all guests");

        // check whether all guests are in the room and joined the expected room
        for (room_id, guests, resps) in result {
            let mut resp = AdminRoomService::get_members(&admin, room_id)
                .await
                .unwrap();
            resp.members.sort();

            assert!(resps.iter().all(Result::is_ok));
            assert!(resps.iter().flatten().all(|r| &r.room_id == room_id));
            assert!(guests.cloned().all(|guest| resp.members.contains(&guest)));
        }

        for sample in samples {
            for (user_id, access_token) in sample.guests() {
                RoomService::kick(
                    &client,
                    access_token,
                    &sample.room_id,
                    RoomKickOrBanBody {
                        reason: Default::default(),
                        user_id: user_id.clone(),
                    },
                )
                .await
                .unwrap();
            }
        }

        // check whether all guests left the room
        for sample in samples {
            let resp = AdminRoomService::get_members(&admin, &sample.room_id)
                .await
                .unwrap();

            assert_eq!(resp.members.len(), 1);
            assert_eq!(
                &[samples
                    .iter()
                    .find(|s| s.room_id == sample.room_id)
                    .map(|s| s.owner())
                    .map(|(id, _)| id.to_owned())
                    .unwrap()],
                resp.members.as_slice()
            );
        }
    }

    #[tokio::test]
    async fn ban_all_guests() {
        let Test { samples, admin, .. } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        // third join
        let result = future::join_all(samples.iter().map(|s| {
            join_helper(&client, s.guests(), &s.room_id)
                .map(|resp| (&s.room_id, s.guests().map(|(id, _)| id), resp))
        }))
        .await;

        tracing::info!("joining all guests");

        // check whether all guests are in the room and joined the expected room
        for (room_id, guests, resps) in result {
            let mut resp = AdminRoomService::get_members(&admin, room_id)
                .await
                .unwrap();
            resp.members.sort();

            assert!(resps.iter().all(Result::is_ok));
            assert!(resps.iter().flatten().all(|r| &r.room_id == room_id));
            assert!(guests.cloned().all(|guest| resp.members.contains(&guest)));
        }

        for sample in samples {
            let (_, owner_token) = sample.owner();

            for (user_id, _) in sample.guests() {
                RoomService::ban(
                    &client,
                    owner_token,
                    &sample.room_id,
                    RoomKickOrBanBody {
                        reason: Default::default(),
                        user_id: user_id.clone(),
                    },
                )
                .await
                .unwrap();
            }
        }

        // fourth join
        let result = future::join_all(samples.iter().map(|s| {
            join_helper(&client, s.guests(), &s.room_id)
                .map(|resp| (&s.room_id, s.guests().map(|(id, _)| id), resp))
        }))
        .await;

        tracing::info!("joining all guests");

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
                    .find(|s| &s.room_id == room_id)
                    .map(|s| s.owner())
                    .map(|(id, _)| id.to_owned())
                    .unwrap()],
                resp.members.as_slice()
            );

            assert!(resps.iter().all(|r| r.is_err()));
        }

        for sample in samples {
            let (_, owner_token) = sample.owner();

            for (user_id, _) in sample.guests() {
                RoomService::unban(
                    &client,
                    owner_token,
                    &sample.room_id,
                    RoomKickOrBanBody {
                        reason: Default::default(),
                        user_id: user_id.clone(),
                    },
                )
                .await
                .unwrap();
            }
        }
    }
}
