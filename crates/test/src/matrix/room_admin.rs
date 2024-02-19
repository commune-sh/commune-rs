#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use futures::{future, TryFutureExt};
    use matrix::{
        admin::resources::room::{ListRoomQuery, MessagesQuery, RoomService as AdminRoomService},
        ruma_common::{RoomId, ServerName},
        ruma_events::{room::name::OriginalRoomNameEvent, AnyTimelineEvent, TimelineEventType},
    };

    use tokio::sync::OnceCell;

    use crate::matrix::util::{self, Test};

    static TEST: OnceCell<Test> = OnceCell::const_new();

    #[tokio::test]
    async fn get_all_rooms() {
        let Test {
            samples,
            server_name,
            admin,
        } = TEST.get_or_init(util::init).await;

        let resp: Vec<_> = AdminRoomService::get_all(admin, ListRoomQuery::default())
            .map_ok(|resp| resp.rooms)
            .await
            .unwrap();

        while let Some(_) = future::try_join_all(resp.iter().map(|r| {
            AdminRoomService::get_room_events(admin, &r.room_id, Default::default())
                .map_ok(|resp| resp.chunk.deserialize().unwrap())
        }))
        .await
        .map(|ok| {
            ok.into_iter().find(|chunk| {
                chunk
                    .iter()
                    .all(|event| event.event_type() != TimelineEventType::RoomName)
            })
        })
        .unwrap()
        {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        let resp: Vec<_> = AdminRoomService::get_all(admin, ListRoomQuery::default())
            .map_ok(|resp| resp.rooms)
            .await
            .unwrap();

        assert_eq!(
            samples
                .iter()
                .map(|s| s.owner())
                .map(|(user_id, _)| {
                    let (id, _) = user_id.localpart().rsplit_once("-").unwrap();
                    Some(format!("{id}-room",))
                })
                .collect::<Vec<_>>(),
            resp.iter().map(|r| r.name.clone()).collect::<Vec<_>>()
        );
        assert_eq!(
            samples
                .iter()
                .map(|s| s.owner())
                .map(|(user_id, _)| {
                    let (id, _) = user_id.localpart().rsplit_once("-").unwrap();
                    Some(format!("#{id}-room-alias:{server_name}",))
                })
                .collect::<Vec<_>>(),
            resp.iter()
                .map(|r| r.canonical_alias.clone())
                .collect::<Vec<_>>()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn get_all_rooms_err() {
        let Test { admin, .. } = TEST.get_or_init(util::init).await;

        let _ = AdminRoomService::get_all(
            admin,
            ListRoomQuery {
                from: Some(u64::MAX),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn get_room_details() {
        let Test {
            samples,
            server_name,
            admin,
        } = TEST.get_or_init(util::init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();
        let (user_id, _) = rand.owner();

        let resp = AdminRoomService::get_one(admin, &rand.room_id)
            .await
            .unwrap();

        let (id, _) = user_id.localpart().rsplit_once("-").unwrap();
        assert_eq!(Some(format!("{id}-room",)), resp.name);
        assert_eq!(
            Some(format!("#{id}-room-alias:{server_name}",)),
            resp.canonical_alias,
        );

        assert_eq!(Some(user_id.to_string()), resp.creator);
        assert_eq!(
            Some(format!("{id}-room-topic",)),
            resp.details.and_then(|d| d.topic),
        );

        assert_eq!(resp.join_rules, Some("public".into()));
        assert!(resp.public);
        assert!(resp.room_type.is_none());
    }

    #[tokio::test]
    #[should_panic]
    async fn get_room_details_err() {
        let Test {
            server_name, admin, ..
        } = TEST.get_or_init(util::init).await;

        let _ = AdminRoomService::get_one(
            admin,
            &RoomId::new(&ServerName::parse(server_name).unwrap()),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn get_room_events() {
        let Test { samples, admin, .. } = TEST.get_or_init(util::init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_room_events(
            admin,
            &rand.room_id,
            // no idea what the type is
            MessagesQuery {
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
        let Test {
            server_name, admin, ..
        } = TEST.get_or_init(util::init).await;

        let _ = AdminRoomService::get_room_events(
            admin,
            <&RoomId>::try_from(server_name.as_str()).unwrap(),
            MessagesQuery {
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
        let Test { samples, admin, .. } = TEST.get_or_init(util::init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_state(admin, &rand.room_id)
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
        let Test {
            server_name, admin, ..
        } = TEST.get_or_init(util::init).await;

        let _ =
            AdminRoomService::get_state(admin, <&RoomId>::try_from(server_name.as_str()).unwrap())
                .await
                .unwrap();
    }

    #[tokio::test]
    async fn get_members() {
        let Test { samples, admin, .. } = TEST.get_or_init(util::init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();
        let (owner_id, _) = rand.owner();

        let resp = AdminRoomService::get_members(admin, &rand.room_id)
            .await
            .unwrap();

        assert_eq!(resp.members, vec![owner_id.to_string()]);
    }

    #[tokio::test]
    #[should_panic]
    async fn get_members_err() {
        let Test {
            server_name, admin, ..
        } = TEST.get_or_init(util::init).await;

        let _ = AdminRoomService::get_members(
            admin,
            <&RoomId>::try_from(server_name.as_str()).unwrap(),
        )
        .await
        .unwrap();
    }
}
