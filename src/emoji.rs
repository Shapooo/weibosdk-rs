#![allow(async_fn_in_trait)]
use serde::Deserialize;
use std::collections::HashMap;

use crate::client::{HttpClient, HttpResponse};
use crate::constants::{
    params::{LANG, UA},
    urls::URL_EMOJI_UPDATE,
};
use crate::err_response::ErrResponse;
use crate::error::{Error, Result};
use crate::utils;
use crate::weibo_api::WeiboAPIImpl;

#[derive(Debug, Clone, Deserialize)]
struct Emoji {
    key: String,
    url: String,
}

#[derive(Debug, Clone, Deserialize)]
struct EmojiData {
    card: Vec<Emoji>,
}

#[derive(Debug, Clone, Deserialize)]
struct EmojiUpdateResponseInner {
    data: EmojiData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum EmojiUpdateResponse {
    Succ(EmojiUpdateResponseInner),
    Fail(ErrResponse),
}

pub trait EmojiUpdateAPI {
    async fn emoji_update(&self) -> Result<HashMap<String, String>>;
}

impl<C: HttpClient> EmojiUpdateAPI for WeiboAPIImpl<C> {
    async fn emoji_update(&self) -> Result<HashMap<String, String>> {
        let params = serde_json::json!({
            "ct": "util",
            "a": "expression_all",
            "user_id": 0,
            "time": utils::get_current_timestamp_millis().to_string(),
            "ua": UA,
            "lang": LANG,
            "version": 6710,
        });

        let response = self
            .client
            .get(URL_EMOJI_UPDATE, &params, self.config.retry_times)
            .await?;
        let res = response.json::<EmojiUpdateResponse>().await?;
        match res {
            EmojiUpdateResponse::Succ(data) => {
                let mut emoji_map = HashMap::new();
                for emoji in data.data.card {
                    emoji_map.insert(emoji.key, emoji.url);
                }
                Ok(emoji_map)
            }
            EmojiUpdateResponse::Fail(err) => Err(Error::ApiError(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mock_client::{MockClient, MockHttpResponse},
        session::Session,
    };
    use std::{io::Read, path::PathBuf};

    #[tokio::test]
    async fn test_emoji_update() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
        };
        let weibo_api = WeiboAPIImpl::from_session(mock_client.clone(), session);

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let testcase_path = PathBuf::from(manifest_dir)
            .join("scripts")
            .join("emoji.json");
        let mut testcase_file = std::fs::File::open(testcase_path).unwrap();
        let mut mock_response_body = String::new();
        testcase_file
            .read_to_string(&mut mock_response_body)
            .unwrap();

        let mock_response = MockHttpResponse::new(200, &mock_response_body);
        mock_client.expect_get(URL_EMOJI_UPDATE, mock_response);

        let emoji_map = weibo_api.emoji_update().await.unwrap();
        assert!(!emoji_map.is_empty());
        assert!(emoji_map.contains_key("[光夜萧逸]"));
        assert_eq!(
            emoji_map.get("[光夜萧逸]").unwrap(),
            "https://d.sinaimg.cn/prd/100/1378/2025/06/24/2025_LoveOsborn_mobile.png"
        );
    }
}

#[cfg(test)]
mod real_tests {
    use super::*;
    use crate::{client, session::Session, weibo_api::WeiboAPIImpl};

    #[tokio::test]
    async fn test_real_emoji_update() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let client = client::new_client_with_headers().unwrap();
            let weibo_api = WeiboAPIImpl::from_session(client, session);
            let res = weibo_api.emoji_update().await;
            assert!(res.is_ok());
            let emoji_map = res.unwrap();
            assert!(!emoji_map.is_empty());
        }
    }
}
