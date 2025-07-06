use anyhow::Context;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::login::SendCode;

//-------------------------------------------------------------
//---------------------- WeiboClient --------------------------
//-------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct WeiboAPI {
    #[serde(skip)]
    client: Client,
    gsid: String,
    uid: String,
    screen_name: String,
}

impl WeiboAPI {
    pub fn new(client: Client, gsid: String, uid: String, screen_name: String) -> Self {
        WeiboAPI {
            client,
            gsid,
            uid,
            screen_name,
        }
    }

    pub fn save_to_json(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let json_data = serde_json::to_string_pretty(self)?;
        fs::write(path, json_data).context("Unable to write to file")
    }

    pub fn load_from_json(path: impl AsRef<Path>, client: Client) -> anyhow::Result<Self> {
        let json_data = fs::read_to_string(path).context("Unable to read from file")?;
        let mut api: WeiboAPI = serde_json::from_str(&json_data)?;
        api.client = client;
        Ok(api)
    }

    pub fn start_login(&self) -> SendCode {
        SendCode::new(self.client.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_and_load_json() {
        let client = Client::new();
        let api = WeiboAPI::new(
            client,
            "test_gsid".to_string(),
            "test_uid".to_string(),
            "test_screen_name".to_string(),
        );
        let path = "test_api.json";

        let save_result = api.save_to_json(path);
        assert!(save_result.is_ok());

        let loaded_api_result = WeiboAPI::load_from_json(path, Client::new());
        assert!(loaded_api_result.is_ok());

        let loaded_api = loaded_api_result.unwrap();
        assert_eq!(api.gsid, loaded_api.gsid);
        assert_eq!(api.uid, loaded_api.uid);
        assert_eq!(api.screen_name, loaded_api.screen_name);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_start_login() {
        let client = Client::new();
        let api = WeiboAPI::new(
            client,
            "test_gsid".to_string(),
            "test_uid".to_string(),
            "test_screen_name".to_string(),
        );
        let send_code = api.start_login();
        // The test mainly checks that the method can be called and returns the correct type.
        // Further testing of SendCode functionality should be in the login module's tests.
        assert!(matches!(send_code, SendCode { .. }));
    }
}
