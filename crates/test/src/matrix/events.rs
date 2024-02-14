#[cfg(test)]
mod tests {
    use std::sync::Once;

    use matrix::{
        admin::resources::room::{MessagesQuery, OrderBy},
        ruma_common::{RoomId, ServerName},
        ruma_events::AnyStateEvent,
    };
    use rand::Rng;
    use tokio::sync::OnceCell;

    #[tokio::test]
    async fn send_room_message() {
        let Test { client, .. } = TEST.get_or_init(util::init).await;

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
}
