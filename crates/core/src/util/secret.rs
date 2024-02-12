use std::{
    borrow::Cow,
    convert::Infallible,
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

/// A `String` wrapper that does not display the value when debugged or
/// displayed.
#[derive(Clone, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub struct Secret(Cow<'static, str>);

impl Secret {
    pub fn new(s: impl Into<Cow<'static, str>>) -> Self {
        Secret(s.into())
    }

    #[inline]
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Returs inner value as [`String`]
    ///
    /// # Shadowing Note
    ///
    /// Intentially shadows [`std::string::ToString::to_string`] to prevent
    /// getting `"[REDACTED]"` when using `to_string`.
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for Secret {
    fn from(s: String) -> Self {
        Secret(Cow::Owned(s))
    }
}

impl FromStr for Secret {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Secret(Cow::Owned(s.to_owned())))
    }
}

impl Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_not_display_value() {
        let secret = Secret::new("secret");
        let display = format!("{}", secret);

        assert_eq!(display, "[REDACTED]");
    }

    #[test]
    fn do_not_debug_value() {
        let secret = Secret::new("secret");
        let display = format!("{:?}", secret);

        assert_eq!(display, "[REDACTED]");
    }

    #[test]
    fn retrieves_original() {
        let secret = Secret::new("secret");
        let value = secret.inner();

        assert_eq!(value, "secret");
    }
}
