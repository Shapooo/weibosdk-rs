#![allow(async_fn_in_trait)]
use log::{debug, error, info};
use serde::Deserialize;

use crate::{
    Post,
    client::{HttpClient, HttpResponse},
    constants::{params::*, urls::*},
    err_response::ErrResponse,
    error::{Error, Result},
    internal::post::PostInternal,
    utils,
    weibo_api::WeiboAPIImpl,
};

#[derive(Debug, Clone, Deserialize)]
struct Card {
    #[allow(unused)]
    card_type: i32,
    mblog: Option<PostInternal>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum ProfileStatusesResponse {
    Succ { cards: Vec<Card> },
    Fail(ErrResponse),
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
            ProfileStatusesResponse::Succ { cards } => {
                debug!("got {} cards", cards.len());
                let posts_iterator = cards.into_iter().filter_map(|card| card.mblog);

                let map_to_post = |post: PostInternal| {
                    post.try_into().map_err(|e: Error| {
                        Error::DataConversionError(format!("post internal to post failed: {e}"))
                    })
                };

                if filter_likes {
                    posts_iterator
                        .filter(|post| post.user.as_ref().is_none_or(|u| u.id == uid))
                        .map(map_to_post)
                        .collect::<Result<Vec<Post>>>()
                } else {
                    posts_iterator
                        .map(map_to_post)
                        .collect::<Result<Vec<Post>>>()
                }
            }
            ProfileStatusesResponse::Fail(err) => {
                error!("failed to get profile statuses: {:?}", err);
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
mod tests {
    use std::{io::Read, path::PathBuf};

    use super::*;
    use crate::{
        mock_client::{MockClient, MockHttpResponse},
        session::Session,
    };

    #[tokio::test]
    async fn test_profile_statuses_ori() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
        };
        let weibo_api = WeiboAPIImpl::from_session(mock_client.clone(), session);

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let testcase_path = PathBuf::from(manifest_dir)
            .join("tests")
            .join("data")
            .join("profile_statuses.json");
        let mut testcase_file = std::fs::File::open(testcase_path).unwrap();
        let mut mock_response_body = String::new();
        testcase_file
            .read_to_string(&mut mock_response_body)
            .unwrap();
        let res = serde_json::from_str::<ProfileStatusesResponse>(&mock_response_body).unwrap();
        let expect_posts = match res {
            ProfileStatusesResponse::Succ { cards } => cards
                .into_iter()
                .filter_map(|card| card.mblog)
                .map(|card| {
                    card.try_into().map_err(|e: Error| {
                        Error::DataConversionError(format!("post internal to post failed: {}", e))
                    })
                })
                .collect::<Result<Vec<Post>>>()
                .unwrap(),
            ProfileStatusesResponse::Fail(_) => panic!("unexpected fail response"),
        };

        let mock_response = MockHttpResponse::new(200, &mock_response_body);
        mock_client.expect_get(URL_PROFILE_STATUSES, mock_response);

        let posts = weibo_api.profile_statuses_original(12345, 1).await.unwrap();
        assert_eq!(posts, expect_posts);
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
            let client = client::new_client_with_headers().unwrap();
            let weibo_api = WeiboAPIImpl::from_session(client, session);
            let posts = weibo_api.profile_statuses(1401527553, 1).await.unwrap();
            assert!(!posts.is_empty());
        }
    }
}
