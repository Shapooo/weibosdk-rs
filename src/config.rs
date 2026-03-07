use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub retry_times: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self { retry_times: 2 }
    }
}
