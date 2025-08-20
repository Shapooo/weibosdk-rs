#![allow(async_fn_in_trait)]
use log::{debug, error, info};
use serde::Deserialize;

use crate::{
    client::{HttpClient, HttpResponse},
    constants::{params::*, urls::URL_STATUSES_SHOW},
    error::{Error, Result},
    models::err_response::ErrResponse,
    utils,
    weibo_api::WeiboAPIImpl,
};

#[derive(Debug, Clone, Deserialize)]
pub struct LongText {
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditConfig {
    #[allow(unused)]
    pub edited: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatusesShow {
    #[allow(unused)]
    pub edit_config: EditConfig,
    #[serde(rename = "longText")]
    pub long_text: LongText,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LongTextResponse {
    Succ(StatusesShow),
    Fail(ErrResponse),
}

pub trait LongTextAPI {
    async fn get_long_text(&self, id: i64) -> Result<String>;
}

impl<C: HttpClient> LongTextAPI for WeiboAPIImpl<C> {
    async fn get_long_text(&self, id: i64) -> Result<String> {
        info!("getting long text, id: {id}");
        let session = self.session()?;
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["id"] = id.into();
        params["isGetLongText"] = 1.into();

        let response = self
            .client
            .get(URL_STATUSES_SHOW, &params, self.config.retry_times)
            .await?;
        let res = response.json::<LongTextResponse>().await?;
        match res {
            LongTextResponse::Succ(statuses_show) => {
                debug!("got long text success");
                Ok(statuses_show.long_text.content)
            }
            LongTextResponse::Fail(err) => {
                error!("failed to get long text: {err:?}");
                Err(Error::ApiError(err))
            }
        }
    }
}

#[cfg(test)]
mod local_tests {
    use std::path::Path;

    use super::*;
    use crate::{
        mock::{MockClient, MockHttpResponse},
        session::Session,
    };

    #[tokio::test]
    async fn test_get_long_text() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
            cookie_store: Default::default(),
        };
        let weibo_api = WeiboAPIImpl::from_session(mock_client.clone(), session);

        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let testcase_path = manifest_dir.join("tests/data/long_text.json");
        let mock_response_body = std::fs::read_to_string(testcase_path).unwrap();
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

#[cfg(test)]
mod real_tests {
    use super::*;
    use crate::{client, session::Session, weibo_api::WeiboAPIImpl};

    #[tokio::test]
    async fn test_real_get_long_text() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let client = client::Client::new().unwrap();
            let weibo_api = WeiboAPIImpl::from_session(client, session);
            let long_text = weibo_api.get_long_text(5179586393932632).await.unwrap();
            assert!(!long_text.is_empty());
        }
    }
}
