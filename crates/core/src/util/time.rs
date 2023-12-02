use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Error;

pub fn timestamp() -> Result<u64, Error> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).map_err(|err| {
        tracing::error!(?err, "Failed to get timestamp");
        Error::Unknown
    })?;

    Ok(since_the_epoch.as_secs())
}
