#[cfg(test)]
mod tests {
    use matrix::{
        admin::resources::room::RoomService as AdminRoomService,
        client::resources::room::{
            ForgetRoomBody, LeaveRoomBody,
            RoomKickOrBanBody, RoomService,
        },
    };
    use tokio::sync::OnceCell;

    use crate::matrix::util::{self, Test, join_helper};

    static TEST: OnceCell<Test> = OnceCell::const_new();

    #[tokio::test]
    async fn join_all_rooms() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let result = join_helper(&client, samples).await;

        let rooms: Vec<_> = result.iter().map(|r| &r.0).collect();
        tracing::info!(?rooms, "joining all guests");

        // check whether all guests are in the room and joined the expected room
        for (room_id, guests, resps) in result {
            let mut resp = AdminRoomService::get_members(&client, &room_id)
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
        let Test {
            samples, admin, ..
        } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        let mut result = Vec::with_capacity(samples.len());

        for sample in samples {
            let guests: Vec<_> = samples
                .iter()
                .filter(|g| g.user_id != sample.user_id)
                .collect();

            for guest in guests {
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
        let Test {
            samples, admin, ..
        } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        for sample in samples {
            let guests: Vec<_> = samples
                .iter()
                .filter(|g| g.user_id != sample.user_id)
                .collect();

            for guest in guests {
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
        let Test {
            samples, admin, ..
        } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        // second join
        let result = join_helper(&client, samples).await;
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
            let room_id = &sample.room_id;

            let guests: Vec<_> = samples
                .iter()
                .filter(|g| g.user_id != sample.user_id)
                .collect();

            for guest in guests {
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
        let Test {
            samples, admin, ..
        } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        // third join
        let result = join_helper(&client, samples).await;
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
            let guests: Vec<_> = samples
                .iter()
                .filter(|g| g.user_id != sample.user_id)
                .collect();

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
        let result = join_helper(&client, samples).await;
        let rooms: Vec<_> = result.iter().map(|r| &r.0).collect();
        tracing::info!(?rooms, "joining all guests");

        // check whether all guests got banned from the room
        // check whether their join request failed
        for (ref room_id, _, resps) in result {
            let resp = AdminRoomService::get_members(&admin, &room_id)
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

            assert!(resps.iter().all(|r| r.is_err()));
        }

        for sample in samples {
            let guests: Vec<_> = samples
                .iter()
                .filter(|g| g.user_id != sample.user_id)
                .collect();

            for guest in guests {
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
