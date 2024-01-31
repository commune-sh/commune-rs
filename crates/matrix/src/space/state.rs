use ruma_common::OwnedEventId;
use ruma_events::EmptyStateKey;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VoteKind {
    Up,
    Down,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "rel_type", rename = "space.board.vote")]
pub struct Vote {
    pub event_id: OwnedEventId,
    pub key: VoteKind,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "space.topic", kind = State, state_key_type = EmptyStateKey)]
pub struct SpaceTopicEventContent {}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "space.restriction", kind = State, state_key_type = EmptyStateKey)]
pub struct SpaceRestrictionEventContent {
    #[ruma_event(skip_redaction)]
    #[serde(flatten)]
    account_age: u32,
}

#[cfg(test)]
mod tests {
    use ruma_common::{exports::serde_json::json, serde::test::serde_json_eq};

    use super::VoteKind;

    #[test]
    fn assert_correct_enum_representation() {
        serde_json_eq(VoteKind::Up, json!("up"));
        serde_json_eq(VoteKind::Down, json!("down"));
    }
}
