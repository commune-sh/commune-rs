use rand::distributions::{Alphanumeric, DistString};
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::secret::Secret;

/// Quantity of elements in each of the parts conforming the verification code,
/// should be a even nember in order to have no remainder when dividing the
/// capacity of the verification code string.
const VERIFICATION_CODE_CHAR: usize = 4;

/// Quantity of parts conforming the verification code, should be a even number
/// in order to have no remainder when dividing the capacity of the verification
/// code string.
const VERIFICATION_CODE_PART: usize = 3;

/// Capacity of the verification code string
const VERIFICATION_CODE_CAPY: usize =
    (VERIFICATION_CODE_PART * VERIFICATION_CODE_CHAR) + VERIFICATION_CODE_PART;

#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationCode {
    pub email: String,
    pub code: Secret,
    pub session: Uuid,
}

impl VerificationCode {
    pub fn new(email: &str, session: &Uuid) -> Self {
        let code = Self::generate_verification_code();

        Self {
            email: email.to_string(),
            code,
            session: *session,
        }
    }

    /// Creates the marshalled representation of the verification code which is
    /// JSON
    pub fn marshall(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Builds an instance of [`VerificationCode`] from the marshalled JSON
    pub fn unmarshall(payload: String) -> Self {
        serde_json::from_str(&payload).unwrap()
    }

    fn generate_verification_code() -> Secret {
        let mut out = String::with_capacity(VERIFICATION_CODE_CAPY - VERIFICATION_CODE_PART);
        let mut rng = rand::prelude::StdRng::from_entropy();

        Alphanumeric.append_string(
            &mut rng,
            &mut out,
            VERIFICATION_CODE_CAPY - VERIFICATION_CODE_PART,
        );

        Secret::from(
            format!("{}-{}-{}", &out[0..=3], &out[4..=7], &out[8..=11]).to_ascii_lowercase(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::VerificationCode;

    #[test]
    fn codes_are_never_repeated() {
        let codes = (1..50)
            .map(|_| VerificationCode::generate_verification_code().to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            codes.len(),
            codes.iter().collect::<std::collections::HashSet<_>>().len()
        );
    }
}
