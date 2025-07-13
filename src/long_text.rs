#![allow(async_fn_in_trait)]
use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::client::{HttpClient, HttpResponse};
use crate::constants::{params::*, urls::URL_STATUSES_SHOW};
use crate::err_response::ErrResponse;
use crate::internal::statuses_show::StatusesShow;
use crate::utils;
use crate::weibo_api::WeiboAPI;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LongTextResponse {
    Succ(StatusesShow),
    Fail(ErrResponse),
}

pub trait LongTextAPI {
    async fn get_long_text(&self, id: i64) -> Result<String>;
}

impl<C: HttpClient> LongTextAPI for WeiboAPI<C> {
    async fn get_long_text(&self, id: i64) -> Result<String> {
        let session = self.session();
        let s = utils::generate_s(&session.uid, FROM);
        let params = serde_json::json!({
            "c": PARAM_C,
            "from": FROM,
            "gsid": &session.gsid,
            "lang": LANG,
            "locale": LOCALE,
            "s": &s,
            "source": SOURCE,
            "ua": UA,
            "wm": WM,
            "id": id,
            "is_show_bulletin": 2,
        });

        let response = self.client.get(URL_STATUSES_SHOW, &params).await?;
        let res = response.json::<LongTextResponse>().await?;
        match res {
            LongTextResponse::Succ(statuses_show) => Ok(statuses_show.long_text.content),
            LongTextResponse::Fail(err) => Err(anyhow!(
                "api call error: {}, {}, {}",
                err.errno,
                err.errmsg,
                err.errtype
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Read, path::PathBuf};

    use super::*;
    use crate::{
        mock_client::{MockClient, MockHttpResponse},
        session::Session,
    };

    #[tokio::test]
    async fn test_get_long_text() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
        };
        let weibo_api = WeiboAPI::new(mock_client.clone(), session);

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let testcase_path = PathBuf::from(manifest_dir)
            .join("tests")
            .join("data")
            .join("long_text.json");
        let mut testcase_file = std::fs::File::open(testcase_path).unwrap();
        let mut mock_response_body = String::new();
        testcase_file
            .read_to_string(&mut mock_response_body)
            .unwrap();
        let expect_long_text = serde_json::from_str::<StatusesShow>(&mock_response_body)
            .unwrap()
            .long_text
            .content;
        let mock_response = MockHttpResponse::new(200, &mock_response_body);
        mock_client.expect_get(URL_STATUSES_SHOW, mock_response);

        let long_text = weibo_api.get_long_text(12345).await.unwrap();
        assert_eq!(long_text, expect_long_text);
    }
}
