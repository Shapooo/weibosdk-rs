use log::debug;

use crate::api_client::ApiClient;
use crate::constants::{
    params::{LANG, UA},
    urls::{URL_EMOJI_UPDATE, URL_WEB_EMOTICON},
};
use crate::error::Result;
use crate::http_client::HttpClient;
use crate::utils;

impl<C: HttpClient> ApiClient<C> {
    pub async fn fetch_from_web_api(&self) -> Result<C::Response> {
        let url = URL_WEB_EMOTICON;
        debug!("fetch emoticon, url: {url}");
        self.client
            .get(url, &serde_json::json!({}), self.config.retry_times)
            .await
    }

    pub async fn fetch_from_mobile_api(&self) -> Result<C::Response> {
        let params = serde_json::json!({
            "ct": "util",
            "a": "expression_all",
            "user_id": 0,
            "time": utils::get_current_timestamp_millis().to_string(),
            "ua": UA,
            "lang": LANG,
            "version": 6710,
        });

        self.client
            .get(URL_EMOJI_UPDATE, &params, self.config.retry_times)
            .await
    }
}

#[cfg(test)]
mod real_tests {
    use crate::{api_client::ApiClient, http_client, session::Session};
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_real_web_emoticon() {
        let session_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("session.json");
        let session = Arc::new(Mutex::new(Session::load(session_file).unwrap()));
        let client = http_client::Client::new().unwrap();
        let mut weibo_api = ApiClient::new(client, Default::default());
        weibo_api.login_with_session(session).await.unwrap();
        let _emoji_map = weibo_api.fetch_from_web_api().await.unwrap();
    }

    #[tokio::test]
    async fn test_real_mobile_emoji() {
        let session_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("session.json");
        let session = Arc::new(Mutex::new(Session::load(session_file).unwrap()));
        let client = http_client::Client::new().unwrap();
        let mut weibo_api = ApiClient::new(client, Default::default());
        weibo_api.login_with_session(session).await.unwrap();
        let _emoji_map = weibo_api.fetch_from_mobile_api().await.unwrap();
    }
}
