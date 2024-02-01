use ruma_events::{EmptyStateKey, StateEventContent};
use ruma_macros::EventContent;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

pub trait SpaceStateEventContent: StateEventContent + DeserializeOwned {}

impl SpaceStateEventContent for SpaceTopicEventContent {}
impl SpaceStateEventContent for SpaceRestrictionEventContent {}
