use matrix::admin::resources::room::{
    ListRoomQuery, ListRoomResponse, RoomService as AdminRoomService,
};

#[cfg(test)]
mod tests {
    use matrix::{
        admin::resources::room::{MessagesQuery, OrderBy},
        ruma_common::{RoomId, ServerName},
    };
    use tokio::sync::OnceCell;

    use crate::matrix::util::{self, Test};

    use super::*;

    static TEST: OnceCell<Test> = OnceCell::const_new();

    #[tokio::test]
    async fn get_all_rooms() {
        let Test {
            samples,
            server_name,
            client,
        } = TEST.get_or_init(util::init).await;

        let ListRoomResponse { rooms: resp, .. } =
            AdminRoomService::get_all(client, ListRoomQuery::default())
                .await
                .unwrap();

        assert_eq!(
            samples
                .iter()
                .map(|s| Some(format!("{id}-room-name", id = s.user_id.localpart())))
                .collect::<Vec<_>>(),
            resp.iter().map(|r| r.name.clone()).collect::<Vec<_>>(),
        );
        assert_eq!(
            samples
                .iter()
                .map(|s| format!("#{id}-room-alias:{server_name}", id = s.user_id.localpart()))
                .collect::<Vec<_>>(),
            resp.iter()
                .map(|r| r.canonical_alias.clone().unwrap())
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            samples.iter().map(|s| &s.room_id).collect::<Vec<_>>(),
            resp.iter().map(|r| &r.room_id).collect::<Vec<_>>(),
        );

        let ListRoomResponse { rooms: resp, .. } = AdminRoomService::get_all(
            client,
            ListRoomQuery {
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
        let Test { client, .. } = TEST.get_or_init(util::init).await;

        let _ = AdminRoomService::get_all(
            client,
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
            client,
        } = TEST.get_or_init(util::init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_one(client, &rand.room_id)
            .await
            .unwrap();

        assert_eq!(
            Some(format!("{}-room-name", rand.user_id.localpart())),
            resp.name
        );
        assert_eq!(
            Some(format!(
                "#{}-room-alias:{server_name}",
                rand.user_id.localpart()
            )),
            resp.canonical_alias,
        );

        assert_eq!(Some(rand.user_id.to_string()), resp.creator);
        assert_eq!(
            Some(format!("{}-room-topic", rand.user_id.localpart())),
            resp.details.and_then(|d| d.topic),
        );

        assert_eq!(resp.join_rules, Some("public".into()));

        assert!(!resp.public);
        assert!(resp.room_type.is_none());
    }

    #[tokio::test]
    #[should_panic]
    async fn get_room_details_err() {
        let Test {
            server_name,
            client,
            ..
        } = TEST.get_or_init(util::init).await;

        let _ = AdminRoomService::get_one(
            client,
            &RoomId::new(&ServerName::parse(server_name).unwrap()),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn get_room_events() {
        let Test {
            samples, client, ..
        } = TEST.get_or_init(util::init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_room_events(
            client,
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
            server_name,
            client,
            ..
        } = TEST.get_or_init(util::init).await;

        let _ = AdminRoomService::get_room_events(
            client,
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
        let Test {
            samples, client, ..
        } = TEST.get_or_init(util::init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_state(client, &rand.room_id)
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
            server_name,
            client,
            ..
        } = TEST.get_or_init(util::init).await;

        let _ =
            AdminRoomService::get_state(client, <&RoomId>::try_from(server_name.as_str()).unwrap())
                .await
                .unwrap();
    }

    #[tokio::test]
    async fn get_members() {
        let Test {
            samples, client, ..
        } = TEST.get_or_init(util::init).await;

        let magic_number = Box::into_raw(Box::new(12345)) as usize % samples.len();
        let rand = samples.get(magic_number).unwrap();

        let resp = AdminRoomService::get_members(client, &rand.room_id)
            .await
            .unwrap();

        assert_eq!(resp.members, vec![rand.user_id.to_string()]);
    }

    #[tokio::test]
    #[should_panic]
    async fn get_members_err() {
        let Test {
            server_name,
            client,
            ..
        } = TEST.get_or_init(util::init).await;

        let _ = AdminRoomService::get_members(
            client,
            <&RoomId>::try_from(server_name.as_str()).unwrap(),
        )
        .await
        .unwrap();
    }
}
