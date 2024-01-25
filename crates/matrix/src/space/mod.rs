use ruma::events::room::message::{deserialize_relation, FormattedBody, Relation};
use ruma::events::Mentions;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "space.board.post", kind = MessageLike, without_relation)]
pub struct BoardPostEventContent {
    /// The title of the post.
    pub title: Option<String>,

    /// The body of the post.
    pub body: String,

    /// Formatted form of the post `body`.
    #[serde(flatten)]
    pub formatted: Option<FormattedBody>,

    /// Information about [related posts].
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_relation"
    )]
    pub relates_to: Option<Relation<BoardPostEventContentWithoutRelation>>,

    /// The mentions of this post.
    #[serde(rename = "m.mentions", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Mentions>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "space.board.reply", kind = MessageLike, without_relation)]
pub struct BoardReplyEventContent {
    /// The body of the reply.
    pub body: String,

    /// Formatted form of the reply `body`.
    #[serde(flatten)]
    pub formatted: Option<FormattedBody>,

    /// Information about [related replies].
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_relation"
    )]
    pub relates_to: Option<Relation<BoardReplyEventContentWithoutRelation>>,

    /// The mentions of this reply.
    #[serde(rename = "m.mentions", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Mentions>,
}
