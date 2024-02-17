#[cfg(test)]
mod tests {
    use std::iter;

    use futures::{future, FutureExt, TryFutureExt};
    use matrix::{
        client::resources::events::{EventsService, GetMessagesQuery},
        filter::RoomEventFilter,
        ruma_common::TransactionId,
        ruma_events::{
            room::message::{
                AddMentions, ForwardThread, OriginalRoomMessageEvent, RoomMessageEventContent,
            },
            MessageLikeEventType,
        },
    };
    use rand::Rng;
    use tokio::sync::OnceCell;

    use crate::matrix::util::{self, join_helper, Test};

    static TEST: OnceCell<Test> = OnceCell::const_new();

    #[tokio::test]
    async fn send_message() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let joins = join_helper(&client, samples).await;

        assert!(joins
            .iter()
            .all(|(_, _, resps)| resps.iter().all(Result::is_ok)));

        future::try_join_all(samples.iter().flat_map(|sample| {
            samples
                .iter()
                .filter(|g| g.user_id != sample.user_id)
                .map(|guest| {
                    EventsService::send_message(
                        &client,
                        guest.access_token.clone(),
                        &sample.room_id,
                        TransactionId::new(),
                        RoomMessageEventContent::text_markdown(format!(
                            "hello, **my name is {}**",
                            guest.user_id
                        )),
                    )
                })
                .chain(iter::once(EventsService::send_message(
                    &client,
                    sample.access_token.clone(),
                    &sample.room_id,
                    TransactionId::new(),
                    RoomMessageEventContent::text_plain(format!(
                        "and I am the admin of the room, {}",
                        sample.user_id
                    )),
                )))
        }))
        .await
        .unwrap();

        let expected: Vec<_> = (0..samples.len())
            .flat_map(|i| {
                (0..samples.len())
                    .map(move |j| match j == i {
                        true => None,
                        false => Some(format!(
                            "hello, **my name is {}**",
                            samples.get(j).map(|s| s.user_id.clone()).unwrap()
                        )),
                    })
                    .chain(iter::once(Some(format!(
                        "and I am the admin of the room, {}",
                        samples.get(i).map(|s| s.user_id.clone()).unwrap()
                    ))))
                    .flatten()
            })
            .collect();

        let filter = RoomEventFilter {
            types: vec![MessageLikeEventType::RoomMessage.into()],
            ..Default::default()
        };

        let filter = serde_json::to_string(&filter).unwrap();

        let found = future::try_join_all(samples.iter().map(|s| {
            EventsService::get_messages(
                &client,
                s.access_token.clone(),
                &s.room_id,
                GetMessagesQuery {
                    limit: Some(111),
                    filter: filter.clone(),
                    ..Default::default()
                },
            )
        }))
        .await
        .unwrap();

        let found: Vec<_> = found
            .into_iter()
            .flat_map(|resp| resp.chunk)
            .map(|e| e.deserialize_as::<OriginalRoomMessageEvent>().unwrap())
            .map(|e| e.content.body().to_owned())
            .collect();

        dbg!(&found, &expected);

        assert!(expected.iter().all(|s| found.contains(s)));
    }

    #[tokio::test]
    async fn reply_to_message() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let joins = join_helper(&client, samples).await;

        assert!(joins
            .iter()
            .all(|(_, _, resps)| resps.iter().all(Result::is_ok)));

        let parent = samples
            .get(rand::thread_rng().gen::<usize>() % samples.len())
            .unwrap();

        let root = EventsService::send_message(
            &client,
            parent.access_token.clone(),
            &parent.room_id,
            TransactionId::new(),
            RoomMessageEventContent::text_plain(format!(
                "I am at the root of the tree, {}",
                parent.user_id
            )),
        )
        .map_ok(move |resp| resp.event_id)
        .await
        .unwrap();

        let n = 5;

        let mut history = Vec::from([vec![root]]);

        for i in 1..n {
            let guest = samples
                .iter()
                .filter(|g| g.user_id != parent.user_id)
                .cycle()
                .nth(i)
                .unwrap();

            let prev = history.last().unwrap();
            let traverse = future::try_join_all((0..prev.len() * 2).map(|j| {
                EventsService::get_event(
                    &client,
                    guest.access_token.clone(),
                    &parent.room_id,
                    prev.get(j / 2).unwrap(),
                )
                .map_ok(|resp| resp.deserialize_as::<OriginalRoomMessageEvent>().unwrap())
                .and_then(|event| {
                    EventsService::send_message(
                        &client,
                        guest.access_token.clone(),
                        &parent.room_id,
                        TransactionId::new(),
                        RoomMessageEventContent::text_markdown(format!("level {i}",))
                            .make_reply_to(&event, ForwardThread::No, AddMentions::Yes),
                    )
                })
                .map_ok(|resp| resp.event_id)
            }))
            .await
            .unwrap();

            history.push(traverse.clone());


            tracing::info!(?traverse);
        }

        let filter = RoomEventFilter {
            types: vec![MessageLikeEventType::RoomMessage.into()],
            ..Default::default()
        };

        let filter = serde_json::to_string(&filter).unwrap();

        let found = EventsService::get_messages(
            &client,
            parent.access_token.clone(),
            &parent.room_id,
                GetMessagesQuery {
                    limit: Some(111),
                    filter: filter.clone(),
                    ..Default::default()
                },
        )
        .map_ok(move |resp| resp.chunk)
        .await
        .unwrap();

        assert!(history.windows(2).all(|events| events[0].len()*2 == events[1].len()));
        dbg!(&found);
    }
}
