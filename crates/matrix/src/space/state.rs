use ruma_events::{EmptyStateKey};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "space.restriction", kind = State, state_key_type = EmptyStateKey)]
pub struct SpaceRestrictionEventContent {
    #[ruma_event(skip_redaction)]
    #[serde(flatten)]
    account_age: u32,
}
