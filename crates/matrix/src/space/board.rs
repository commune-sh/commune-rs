use ruma::events::room::message::{FormattedBody, Relation};
use ruma::events::Mentions;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum BoardType {
    Post(BoardPostEventContent),
    // not sure if this should be unit
    _Custom(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "boardtype", rename = "space.board.post")]
pub struct BoardPostEventContent {
    /// The title of the post.
    pub title: Option<String>,

    /// The body of the post.
    pub body: String,

    /// Formatted form of the post `body`.
    #[serde(flatten)]
    pub formatted: Option<FormattedBody>,
}

#[derive(Clone, Debug, Serialize, EventContent)]
#[ruma_event(type = "space.board", kind = MessageLike)]
pub struct SpaceBoardContent {
    #[serde(flatten)]
    pub boardtype: BoardType,

    /// Information about [related posts].
    ///
    /// [related posts]: https://spec.matrix.org/latest/client-server-api/#forming-relationships-between-events
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<Relation<SpaceBoardContentWithoutRelation>>,

    /// The [mentions] of this event.
    ///
    /// This should always be set to avoid triggering the legacy mention push rules. It is
    /// recommended to use [`Self::set_mentions()`] to set this field, that will take care of
    /// populating the fields correctly if this is a replacement.
    ///
    /// [mentions]: https://spec.matrix.org/latest/client-server-api/#user-and-room-mentions
    #[serde(rename = "m.mentions", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Mentions>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SpaceBoardContentWithoutRelation {
    #[serde(flatten)]
    pub boardtype: BoardType,

    /// The [mentions] of this event.
    ///
    /// This should always be set to avoid triggering the legacy mention push rules. It is
    /// recommended to use [`Self::set_mentions()`] to set this field, that will take care of
    /// populating the fields correctly if this is a replacement.
    ///
    /// [mentions]: https://spec.matrix.org/latest/client-server-api/#user-and-room-mentions
    #[serde(rename = "m.mentions", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Mentions>,
}
