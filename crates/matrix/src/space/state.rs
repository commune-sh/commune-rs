use ruma_events::EmptyStateKey;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VoteKind {
    Up,
    Down,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "space.board.vote", kind = State, state_key_type = String)]
pub struct BoardVoteEventContent {
    #[serde(rename = "type")]
    kind: VoteKind,
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
    use ruma_common::{serde::test::serde_json_eq, exports::serde_json::json};

    use super::VoteKind;


    #[test]
    fn assert_correct_enum_representation() {
        serde_json_eq(VoteKind::Up, json!("up"));
        serde_json_eq(VoteKind::Down, json!("down"));
    }
}
