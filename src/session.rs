use std::{fs, path::Path};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub gsid: String,
    pub uid: String,
    pub screen_name: String,
    // Add other session-related fields here if necessary
}

impl Session {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Loading session from {:?}", path.as_ref());
        let content = fs::read_to_string(path)?;
        let session: Session = serde_json::from_str(&content)?;
        debug!(
            "Session loaded successfully for user {}",
            session.screen_name
        );
        Ok(session)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        info!("Saving session to {:?}", path.as_ref());
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        debug!("Session saved successfully for user {}", self.screen_name);
        Ok(())
    }
}
