use std::fmt::{Debug, Display};

use rand::{distributions::Uniform, Rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Secret(String);

// is this necessary?
impl Serialize for Secret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner().serialize(serializer)
    }
}

impl Secret {
    #[inline]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    #[inline]
    pub fn inner(&self) -> String {
        self.0.clone()
    }
}

impl Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{self}").as_str())
    }
}

impl Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let braille_range = Uniform::new('\u{2800}', '\u{28FF}');
        let s: String = rand::thread_rng()
            .sample_iter(braille_range)
            .take(self.0.len())
            .collect();

        f.write_str(s.as_str())
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

        assert_eq!(value, "secret".into());
    }
}
