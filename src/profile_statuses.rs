#![allow(async_fn_in_trait)]
use log::{debug, error, info};
use serde::Deserialize;

use crate::{
    Post,
    client::{HttpClient, HttpResponse},
    constants::{params::*, urls::*},
    err_response::ErrResponse,
    error::{Error, Result},
    utils,
    weibo_api::WeiboAPIImpl,
};

#[derive(Debug, Clone, Deserialize)]
struct Card {
    #[allow(unused)]
    card_type: i32,
    mblog: Option<Post>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum ProfileStatusesResponse {
    Succ(ProfileStatusesSucc),
    Fail(ErrResponse),
}

#[derive(Debug, Clone, Deserialize)]
struct ProfileStatusesSucc {
    cards: Vec<Card>,
}

pub trait ProfileStatusesAPI {
    async fn profile_statuses(&self, uid: i64, page: u32) -> Result<Vec<Post>>;
    async fn profile_statuses_original(&self, uid: i64, page: u32) -> Result<Vec<Post>>;
    async fn profile_statuses_picture(&self, uid: i64, page: u32) -> Result<Vec<Post>>;
    async fn profile_statuses_video(&self, uid: i64, page: u32) -> Result<Vec<Post>>;
    async fn profile_statuses_article(&self, uid: i64, page: u32) -> Result<Vec<Post>>;
}

impl<C: HttpClient> WeiboAPIImpl<C> {
    async fn get_profile_statuses(
        &self,
        uid: i64,
        page: u32,
        containerid: String,
        filter_likes: bool,
    ) -> Result<Vec<Post>> {
        info!(
            "getting profile statuses, uid: {}, page: {}, containerid: {}",
            uid,
            page,
            containerid.clone()
        );
        let session = self.session()?;
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["uid"] = uid.into();
        params["page"] = page.into();
        params["count"] = self.config.status_count.into();
        params["mix_media_enable"] = MIX_MEDIA_ENABLE.into();
        params["containerid"] = containerid.into();
        let response = self
            .client
            .get(URL_PROFILE_STATUSES, &params, self.config.retry_times)
            .await?;
        let response = response.json::<ProfileStatusesResponse>().await?;
        match response {
            ProfileStatusesResponse::Succ(ProfileStatusesSucc { cards }) => {
                let posts_iterator = cards.into_iter().filter_map(|card| card.mblog);

                let posts = if filter_likes {
                    posts_iterator
                        .filter(|post| post.user.as_ref().is_none_or(|u| u.id == uid))
                        .collect::<Vec<Post>>()
                } else {
                    posts_iterator.collect::<Vec<Post>>()
                };
                debug!("got {} posts", posts.len());
                Ok(posts)
            }
            ProfileStatusesResponse::Fail(err) => {
                error!("failed to get profile statuses: {err:?}");
                Err(Error::ApiError(err))
            }
        }
    }
}

impl<C: HttpClient> ProfileStatusesAPI for WeiboAPIImpl<C> {
    async fn profile_statuses(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        let containerid = format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO");
        self.get_profile_statuses(uid, page, containerid, true)
            .await
    }

    async fn profile_statuses_original(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        let containerid = format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO_ORI");
        self.get_profile_statuses(uid, page, containerid, false)
            .await
    }

    async fn profile_statuses_picture(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        let containerid = format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO_PIC");
        self.get_profile_statuses(uid, page, containerid, false)
            .await
    }

    async fn profile_statuses_video(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        let containerid = format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO_VIDEO");
        self.get_profile_statuses(uid, page, containerid, false)
            .await
    }

    async fn profile_statuses_article(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        let containerid = format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO_ARTICAL");
        self.get_profile_statuses(uid, page, containerid, false)
            .await
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
    async fn test_profile_statuses_ori() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
            cookie_store: Default::default(),
        };
        let weibo_api = WeiboAPIImpl::from_session(mock_client.clone(), session);

        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let testcase_path = manifest_dir.join("tests/data/profile_statuses.json");
        let mock_response_body = std::fs::read_to_string(testcase_path).unwrap();

        let mock_response = MockHttpResponse::new(200, &mock_response_body);
        mock_client.expect_get(URL_PROFILE_STATUSES, mock_response);

        weibo_api.profile_statuses_original(12345, 1).await.unwrap();
    }
}

#[cfg(test)]
mod real_tests {
    use super::*;
    use crate::{client, session::Session, weibo_api::WeiboAPIImpl};

    #[tokio::test]
    async fn test_real_profile_statuses() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let client = client::Client::new().unwrap();
            let weibo_api = WeiboAPIImpl::from_session(client, session);
            let posts = weibo_api.profile_statuses(1401527553, 1).await.unwrap();
            assert!(!posts.is_empty());
        }
    }
}
