use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Helper module for serializing/deserializing `std::time::Duration` as seconds.
mod duration_as_secs {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Config {
    pub retry_times: u8,
    #[serde(with = "duration_as_secs")]
    pub timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            retry_times: 2,
            timeout: Duration::from_secs(10),
        }
    }
}
