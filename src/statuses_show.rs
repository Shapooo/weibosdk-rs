#![allow(async_fn_in_trait)]
use log::{debug, error, info};
use serde::Deserialize;

use crate::models::post::Post;

use crate::{
    client::{HttpClient, HttpResponse},
    constants::{params::*, urls::URL_STATUSES_SHOW},
    error::{Error, Result},
    models::err_response::ErrResponse,
    utils,
    weibo_api::WeiboAPIImpl,
};

#[derive(Debug, Clone, Deserialize)]
pub struct EditConfig {
    #[allow(unused)]
    pub edited: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StatusesShowResponse {
    Succ(Post),
    Fail(ErrResponse),
}

pub trait StatusesShowAPI {
    async fn statuses_show(&self, id: i64) -> Result<Post>;
}

impl<C: HttpClient> StatusesShowAPI for WeiboAPIImpl<C> {
    async fn statuses_show(&self, id: i64) -> Result<Post> {
        info!("getting long text, id: {id}");
        let session = self.session()?;
        let session = session.lock().unwrap().clone();
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
        let res = response.json::<StatusesShowResponse>().await?;
        match res {
            StatusesShowResponse::Succ(statuses_show) => {
                debug!("got statuses success");
                Ok(statuses_show)
            }
            StatusesShowResponse::Fail(err) => {
                error!("failed to get long text: {err:?}");
                Err(Error::ApiError(err))
            }
        }
    }
}

#[cfg(test)]
mod local_tests {
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::{
        mock::{MockClient, MockHttpResponse},
        session::Session,
    };

    #[tokio::test]
    async fn test_get_statuses_show() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
            cookie_store: Default::default(),
        };
        let session = Arc::new(Mutex::new(session));
        let weibo_api = WeiboAPIImpl::from_session(mock_client.clone(), session);

        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let testcase_path = manifest_dir.join("tests/data/statuses_show.json");
        let mock_response_body = std::fs::read_to_string(testcase_path).unwrap();
        let mock_response = MockHttpResponse::new(200, &mock_response_body);
        mock_client.expect_get(URL_STATUSES_SHOW, mock_response);

        let _post = weibo_api.statuses_show(12345).await.unwrap();
    }
}

#[cfg(test)]
mod real_tests {
    use super::*;
    use crate::{client, session::Session, weibo_api::WeiboAPIImpl};
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_real_get_statuses_show() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let session = Arc::new(Mutex::new(session));
            let client = client::Client::new().unwrap();
            let weibo_api = WeiboAPIImpl::from_session(client, session);
            let post = weibo_api.statuses_show(5179586393932632).await.unwrap();
            assert!(!post.long_text.unwrap().is_empty());
        }
    }
}
