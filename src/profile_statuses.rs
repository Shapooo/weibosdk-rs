#![allow(async_fn_in_trait)]
use serde::Deserialize;

use crate::Post;
use crate::client::{HttpClient, HttpResponse};
use crate::constants::{params::*, urls::*};
use crate::err_response::ErrResponse;
use crate::error::{Error, Result};
use crate::internal::post::PostInternal;
use crate::utils;
use crate::weibo_api::WeiboAPI;

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
}

impl<C: HttpClient> ProfileStatusesAPI for WeiboAPI<C> {
    async fn profile_statuses(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        let session = self.session();
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["uid"] = uid.into();
        params["page"] = page.into();
        params["count"] = COUNT.into();
        params["mix_media_enable"] = MIX_MEDIA_ENABLE.into();
        let response = self.client.get(URL_PROFILE_STATUSES, &params).await?;
        let response = response.json::<ProfileStatusesResponse>().await?;
        match response {
            ProfileStatusesResponse::Succ { cards } => Ok(cards
                .into_iter()
                .filter_map(|card| card.mblog)
                .map(|post| {
                    post.try_into().map_err(|e: Error| {
                        Error::DataConversionError(format!("post internal to post failed: {}", e))
                    })
                })
                .collect::<Result<Vec<Post>>>()?),
            ProfileStatusesResponse::Fail(err) => Err(Error::ApiError(err)),
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
    async fn test_profile_statuses() {
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

        let posts = weibo_api.profile_statuses(12345, 1).await.unwrap();
        assert_eq!(posts, expect_posts);
    }
}
