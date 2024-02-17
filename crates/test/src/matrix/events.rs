#[cfg(test)]
mod tests {
    use std::iter;

    use futures::{future, TryFutureExt};
    use matrix::{
        client::resources::events::{EventsService, GetMessagesQuery, SendRedactionBody},
        filter::RoomEventFilter,
        ruma_common::{RoomVersionId, TransactionId},
        ruma_events::{
            reaction::{OriginalReactionEvent, ReactionEventContent},
            relation::{Annotation, InReplyTo},
            room::{
                message::{
                    AddMentions, ForwardThread, OriginalRoomMessageEvent, Relation,
                    RoomMessageEvent, RoomMessageEventContent,
                },
                redaction::OriginalRoomRedactionEvent,
                topic::{OriginalRoomTopicEvent, RoomTopicEventContent},
            },
            MessageLikeEvent, MessageLikeEventType,
        },
    };
    use tokio::sync::OnceCell;

    use crate::matrix::util::{self, join_helper, Test};

    static TEST: OnceCell<Test> = OnceCell::const_new();

    #[tokio::test]
    async fn send_message() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;
        let sample = samples.get(0).unwrap();
        let (owner_id, owner_token) = sample.owner();

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let joins = join_helper(&client, sample.guests(), &sample.room_id).await;

        assert!(joins.iter().all(Result::is_ok));

        future::try_join_all(
            sample
                .guests()
                .map(|(user_id, access_token)| {
                    EventsService::send_message(
                        &client,
                        access_token,
                        &sample.room_id,
                        TransactionId::new(),
                        RoomMessageEventContent::text_markdown(format!(
                            "hello, **my name is {}**",
                            user_id
                        )),
                    )
                })
                .chain(iter::once(EventsService::send_message(
                    &client,
                    owner_token,
                    &sample.room_id,
                    TransactionId::new(),
                    RoomMessageEventContent::text_plain(format!(
                        "and I am the admin of the room, {}",
                        owner_id
                    )),
                ))),
        )
        .await
        .unwrap();

        let expected: Vec<_> = sample
            .guests()
            .map(|(user_id, _)| format!("hello, **my name is {}**", user_id))
            .chain(iter::once(format!(
                "and I am the admin of the room, {}",
                owner_id
            )))
            .collect();

        let found = EventsService::get_messages(
            &client,
            owner_token,
            &sample.room_id,
            GetMessagesQuery {
                limit: Some(111),
                filter: serde_json::to_string(&RoomEventFilter {
                    types: vec![MessageLikeEventType::RoomMessage.into()],
                    ..Default::default()
                })
                .unwrap(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        let found: Vec<_> = found
            .chunk
            .into_iter()
            .map(|e| e.deserialize_as::<OriginalRoomMessageEvent>().unwrap())
            .map(|e| e.content.body().to_owned())
            .collect();

        assert!(expected.iter().all(|s| found.contains(s)));
    }

    #[tokio::test]
    async fn reply_to_message() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;
        let sample = samples.get(2).unwrap();
        let (owner_id, owner_token) = sample.owner();

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let joins = join_helper(&client, sample.guests(), &sample.room_id).await;
        assert!(joins.iter().all(Result::is_ok));

        let root = EventsService::send_message(
            &client,
            owner_token,
            &sample.room_id,
            TransactionId::new(),
            RoomMessageEventContent::text_plain(format!(
                "I am at the root of the tree, {}",
                owner_id
            )),
        )
        .map_ok(|resp| resp.event_id)
        .await
        .unwrap();

        let recursion = 5;
        let children = 2;

        let mut history = Vec::from([vec![root]]);

        for level in 1..recursion {
            let guests: Vec<_> = sample.guests().collect();
            let (_, access_token) = guests.get((recursion - 1) % guests.len()).unwrap();

            let prev = history.last().unwrap();
            let traverse = future::try_join_all((0..prev.len() * children).map(|i| {
                EventsService::get_event(
                    &client,
                    *access_token,
                    &sample.room_id,
                    prev.get(i / children).unwrap(),
                )
                .map_ok(|resp| resp.deserialize_as::<OriginalRoomMessageEvent>().unwrap())
                .and_then(|event| {
                    EventsService::send_message(
                        &client,
                        *access_token,
                        &sample.room_id,
                        TransactionId::new(),
                        RoomMessageEventContent::text_markdown(format!("level {level}"))
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

        let filter = serde_json::to_string(&RoomEventFilter {
            types: vec![MessageLikeEventType::RoomMessage.into()],
            ..Default::default()
        })
        .unwrap();

        let found: Vec<_> = EventsService::get_messages(
            &client,
            owner_token,
            &sample.room_id,
            GetMessagesQuery {
                limit: Some(111),
                filter: filter.clone(),
                ..Default::default()
            },
        )
        .map_ok(|resp| {
            resp.chunk
                .into_iter()
                .map(|e| e.deserialize_as::<OriginalRoomMessageEvent>().unwrap())
                .map(|e| {
                    (
                        e.event_id,
                        e.content.body().to_owned(),
                        e.content.relates_to,
                    )
                })
                .collect()
        })
        .await
        .unwrap();

        // this is just `map (n -> n - 1) [1, 2 , 4, 8, ...]`
        let v: Vec<_> = (0..recursion)
            .map(|i| children.pow(i as u32) as usize - 1)
            .collect();

        let tree: Vec<_> = v
            .windows(2)
            .map(|arr| (arr[0], arr[1]))
            .map(|(i, j)| found[i..j].to_vec())
            .collect();

        assert!(tree
            .windows(2)
            .all(|events| events[0].len() * 2 == events[1].len()));

        let ok = tree
            .windows(2)
            .map(|arr| (arr[0].clone(), arr[1].clone()))
            .all(|(parents, children)| {
                children
                    .iter()
                    .map(|(_, _, relation)| relation.clone().unwrap())
                    .all(|relation| match relation {
                        Relation::Reply {
                            in_reply_to: InReplyTo { event_id, .. },
                        } => parents
                            .iter()
                            .find(|(parent_id, _, _)| parent_id == &event_id)
                            .is_some(),
                        _ => panic!(),
                    })
            });

        assert!(ok);
    }

    #[tokio::test]
    async fn redact_message() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;
        let sample = samples.get(3).unwrap();
        let (owner_id, owner_token) = sample.owner();

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let joins = join_helper(&client, sample.guests(), &sample.room_id).await;

        assert!(joins.iter().all(Result::is_ok));

        let messages = future::try_join_all(
            sample
                .guests()
                .map(|(user_id, access_token)| {
                    EventsService::send_message(
                        &client,
                        access_token,
                        &sample.room_id,
                        TransactionId::new(),
                        RoomMessageEventContent::text_markdown(format!(
                            "hello, **my name is {}**",
                            user_id
                        )),
                    )
                })
                .chain(iter::once(EventsService::send_message(
                    &client,
                    owner_token,
                    &sample.room_id,
                    TransactionId::new(),
                    RoomMessageEventContent::text_plain(format!(
                        "and I am the admin of the room, {}",
                        owner_id
                    )),
                ))),
        )
        .await
        .unwrap();

        future::try_join_all(messages[..sample.user_ids.len() - 1].iter().map(|resp| {
            EventsService::send_redaction(
                &client,
                owner_token,
                &sample.room_id,
                &resp.event_id,
                TransactionId::new(),
                SendRedactionBody {
                    reason: format!("I don't like your tone"),
                },
            )
        }))
        .await
        .unwrap();

        let messages: Vec<_> = EventsService::get_messages(
            &client,
            owner_token,
            &sample.room_id,
            GetMessagesQuery {
                limit: Some(111),
                filter: serde_json::to_string(&RoomEventFilter {
                    types: vec![MessageLikeEventType::RoomMessage.into()],
                    not_senders: vec![owner_id.to_owned()],
                    ..Default::default()
                })
                .unwrap(),
                ..Default::default()
            },
        )
        .map_ok(|resp| {
            resp.chunk
                .into_iter()
                .map(|e| e.deserialize_as::<RoomMessageEvent>().unwrap())
                .collect()
        })
        .await
        .unwrap();

        let redactions: Vec<_> = EventsService::get_messages(
            &client,
            owner_token,
            &sample.room_id,
            GetMessagesQuery {
                limit: Some(111),
                filter: serde_json::to_string(&RoomEventFilter {
                    types: vec![MessageLikeEventType::RoomRedaction.into()],
                    ..Default::default()
                })
                .unwrap(),
                ..Default::default()
            },
        )
        .map_ok(|resp| {
            resp.chunk
                .into_iter()
                .map(|e| e.deserialize_as::<OriginalRoomRedactionEvent>().unwrap())
                .collect()
        })
        .await
        .unwrap();

        assert!(messages[..sample.user_ids.len() - 1]
            .iter()
            .all(|m| m.as_original().is_none()));

        assert!(messages[..sample.user_ids.len() - 1]
            .iter()
            .all(|m| redactions
                .iter()
                .find(|r| r.redacts(&RoomVersionId::V11) == m.event_id() && &r.sender == owner_id)
                .is_some()));

        assert!(messages[..sample.user_ids.len() - 1]
            .iter()
            .all(|m| match m {
                MessageLikeEvent::Redacted(_) => true,
                _ => false,
            }));
    }

    #[tokio::test]
    async fn annotate_message() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;
        let sample = samples.get(3).unwrap();
        let (owner_id, owner_token) = sample.owner();

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let joins = join_helper(&client, sample.guests(), &sample.room_id).await;

        assert!(joins.iter().all(Result::is_ok));

        let message = EventsService::send_message(
            &client,
            owner_token,
            &sample.room_id,
            TransactionId::new(),
            RoomMessageEventContent::text_plain(format!(
                "and I am the admin of the room, {}",
                owner_id
            )),
        )
        .await
        .unwrap();

        future::try_join_all(sample.guests().map(|(_, access_token)| {
            EventsService::send_message(
                &client,
                access_token,
                &sample.room_id,
                TransactionId::new(),
                ReactionEventContent::new(Annotation::new(
                    message.event_id.to_owned(),
                    "owo".to_owned(),
                )),
            )
        }))
        .await
        .unwrap();

        let annotations: Vec<_> = EventsService::get_messages(
            &client,
            owner_token,
            &sample.room_id,
            GetMessagesQuery {
                limit: Some(111),
                filter: serde_json::to_string(&RoomEventFilter {
                    types: vec![MessageLikeEventType::Reaction.into()],
                    ..Default::default()
                })
                .unwrap(),
                ..Default::default()
            },
        )
        .map_ok(|resp| {
            resp.chunk
                .into_iter()
                .map(|e| e.deserialize_as::<OriginalReactionEvent>().unwrap())
                .collect()
        })
        .await
        .unwrap();

        assert!(annotations
            .iter()
            .all(|m| m.content.relates_to.event_id == message.event_id
                && m.content.relates_to.key == "owo".to_owned()));
    }

    #[tokio::test]
    async fn send_state() {
        let Test { admin, samples, .. } = TEST.get_or_init(util::init).await;
        let sample = samples.get(4).unwrap();
        let (_, owner_token) = sample.owner();

        let mut client = admin.clone();
        client.clear_token();

        // first join
        let joins = join_helper(&client, sample.guests(), &sample.room_id).await;

        assert!(joins.iter().all(Result::is_ok));

        let _ = EventsService::send_state(
            &client,
            owner_token,
            &sample.room_id,
            None,
            RoomTopicEventContent::new("secret banana party".to_owned()),
        )
        .await
        .unwrap();

        let state: Vec<_> = EventsService::get_state(&client, owner_token, &sample.room_id)
            .map_ok(|resp| {
                resp.0
                    .iter()
                    .filter_map(|e| e.deserialize_as::<OriginalRoomTopicEvent>().ok())
                    .collect()
            })
            .await
            .unwrap();

        assert!(state
            .iter()
            .find(|s| s.content.topic == "secret banana party".to_owned())
            .is_some());
    }
}
