#[cfg(test)]
mod tests {
    use std::iter;

    use futures::{future,};
    use matrix::{
        client::resources::events::{EventsService, GetMessagesQuery, },
        filter::{RoomEventFilter},
        ruma_common::TransactionId,
        ruma_events::{
            room::message::{OriginalRoomMessageEvent, RoomMessageEventContent},
            MessageLikeEventType,
        },
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

        future::try_join_all(samples.iter().flat_map(|sample| {
            samples.iter()
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
            }).chain(iter::once(
            EventsService::send_message(
                &client,
                sample.access_token.clone(),
                &sample.room_id,
                TransactionId::new(),
                RoomMessageEventContent::text_plain(format!(
                    "and I am the admin of the room, {}",
                    sample.user_id
                )),
            )))
        })).await.unwrap();

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
                    .chain([Some(format!(
                        "and I am the admin of the room, {}",
                        samples.get(i).map(|s| s.user_id.clone()).unwrap()
                    ))])
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

        tracing::info!(?found);

        assert_eq!(expected, found);
    }
}
