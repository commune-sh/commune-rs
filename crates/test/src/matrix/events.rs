#[cfg(test)]
mod tests {
    use futures::{future, TryFutureExt};
    use matrix::{
        client::resources::events::{EventsService, GetMessagesQuery, SendMessageResponse},
        filter::{Filter, FilterService, RoomEventFilter},
        ruma_common::TransactionId,
        ruma_events::{room::message::RoomMessageEventContent, MessageLikeEventType},
    };
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

        let mut events = Vec::with_capacity(samples.len());

        for sample in samples {
            let room_id = &sample.room_id;

            let guests: Vec<_> = samples
                .iter()
                .filter(|g| g.user_id != sample.user_id)
                .collect();
            let mut resps = Vec::with_capacity(samples.len());

            for guest in guests.iter() {
                let SendMessageResponse { event_id } = EventsService::send_message(
                    &client,
                    guest.access_token.clone(),
                    room_id,
                    &TransactionId::new(),
                    RoomMessageEventContent::text_markdown(format!(
                        "hello, **my name is {}**",
                        guest.user_id
                    )),
                )
                .await
                .unwrap();

                tracing::info!(
                    "{} sent an event with ID {} to {}",
                    guest.user_id,
                    event_id,
                    room_id
                );

                resps.push(event_id);
            }

            let SendMessageResponse { event_id } = EventsService::send_message(
                &client,
                sample.access_token.clone(),
                room_id,
                &TransactionId::new(),
                RoomMessageEventContent::text_plain(format!(
                    "and I am the admin of the room, {}",
                    sample.user_id
                )),
            )
            .await
            .unwrap();

            resps.push(event_id);
            events.push((room_id, resps));
        }

        let expected: Vec<_> = (0..samples.len())
            .flat_map(|i| {
                (0..samples.len()).map(move |j| match j == i {
                    true => format!(
                        "and I am the admin of the room, {}",
                        samples.get(j).map(|s| s.user_id.clone()).unwrap()
                    ),
                    false => format!(
                        "hello, **my name is {}**",
                        samples.get(j).map(|s| s.user_id.clone()).unwrap()
                    ),
                })
            })
            .collect();

        let filter = RoomEventFilter {
            types: vec![MessageLikeEventType::RoomMessage.into()],
            ..Default::default()
        };

        let filter = serde_json::to_string(&filter).unwrap();
        // dbg!(&s);

        client.clear_token();
        let found = future::try_join_all(samples.iter().map(|s| {
                EventsService::get_messages(
                    &client,
                    s.access_token.clone(),
                    &s.room_id,
                    GetMessagesQuery {
                        // limit: Some(100),
                        dir: matrix::admin::resources::room::Direction::Backward,
                        filter: filter.clone(),
                        ..Default::default()
                    },
                )
        }))
        .await
        .unwrap();

        dbg!(&found);

        // tracing::info!(?found);
        // tracing::info!(?expected);

        // assert_eq!(expected, found);
    }
}
