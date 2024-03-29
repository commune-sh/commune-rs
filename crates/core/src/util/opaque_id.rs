use std::{fmt::Display, str::FromStr};

use matrix::ruma_identifiers_validation::Error;
use serde::{de, Deserialize, Deserializer};

// helper type to validate the opaque part of a room ID separately.
pub struct OpaqueId(String);

impl Display for OpaqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl FromStr for OpaqueId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().all(char::is_alphanumeric) {
            true => Ok(OpaqueId(s.to_owned())),
            false => Err(Error::InvalidCharacters),
        }
    }
}

impl<'de> Deserialize<'de> for OpaqueId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}
