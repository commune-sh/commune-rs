use ruma::events::Mentions;
use serde::{Deserialize, de};
use ruma_common::serde::from_raw_json_value;
use serde_json::value::RawValue as RawJsonValue;

use super::board::{BoardType, SpaceBoardContentWithoutRelation};

impl<'de> Deserialize<'de> for SpaceBoardContentWithoutRelation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let json = Box::<RawJsonValue>::deserialize(deserializer)?;

        let MentionsDeHelper { mentions } = from_raw_json_value(&json)?;

        Ok(Self { boardtype: from_raw_json_value(&json)?, mentions })
    }
}

impl<'de> Deserialize<'de> for BoardType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let json = Box::<RawJsonValue>::deserialize(deserializer)?;
        let BoardTypeDeHelper { boardtype } = from_raw_json_value(&json)?;

        Ok(match boardtype.as_ref() {
            "space.board.post" => Self::Post(from_raw_json_value(&json)?),
            _ => Self::_Custom(()),
        })
    }
}

/// Helper struct to determine the msgtype from a `serde_json::value::RawValue`
#[derive(Debug, Deserialize)]
struct BoardTypeDeHelper {
    /// The message type field
    boardtype: String,
}

#[derive(Deserialize)]
struct MentionsDeHelper {
    #[serde(rename = "m.mentions")]
    mentions: Option<Mentions>,
}
