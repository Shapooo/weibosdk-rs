use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conifg {
    pub fav_count: u8,
    pub status_count: u8,
    pub retry_times: u8,
}

impl Default for Conifg {
    fn default() -> Self {
        Self {
            fav_count: 20,
            status_count: 20,
            retry_times: 2,
        }
    }
}
