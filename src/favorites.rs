#![allow(async_fn_in_trait)]
use serde::Deserialize;

use crate::Post;
use crate::client::{HttpClient, HttpResponse};
use crate::constants::{
    params::*,
    urls::{URL_FAVORITES, URL_FAVORITES_DESTROY},
};
use crate::err_response::ErrResponse;
use crate::error::{Error, Result};
use crate::internal::post::PostInternal;
use crate::utils;
use crate::weibo_api::WeiboAPI;

#[derive(Debug, Clone, Deserialize)]
struct FavoritesPost {
    pub status: PostInternal,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum FavoritesResponse {
    Succ { favorites: Vec<FavoritesPost> },
    Fail(ErrResponse),
}

pub trait FavoritesAPI {
    async fn favorites(&self, page: u32) -> Result<Vec<Post>>;
    async fn favorites_destroy(&self, id: i64) -> Result<()>;
}

impl<C: HttpClient> FavoritesAPI for WeiboAPI<C> {
    async fn favorites(&self, page: u32) -> Result<Vec<Post>> {
        let session = self.session();
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["page"] = page.into();
        params["count"] = COUNT.into();
        params["mix_media_enable"] = MIX_MEDIA_ENABLE.into();

        let response = self.client.get(URL_FAVORITES, &params).await?;
        let res = response.json::<FavoritesResponse>().await?;
        match res {
            FavoritesResponse::Succ { favorites } => {
                let posts = favorites
                    .into_iter()
                    .map(|post| {
                        post.status.try_into().map_err(|e: Error| {
                            Error::DataConversionError(format!(
                                "post internal to post failed: {}",
                                e
                            ))
                        })
                    })
                    .collect::<Result<Vec<Post>>>()?;
                Ok(posts)
            }
            FavoritesResponse::Fail(err) => Err(Error::ApiError(err)),
        }
    }

    async fn favorites_destroy(&self, id: i64) -> Result<()> {
        let session = self.session();
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["id"] = id.into();
        let _ = self.client.post(URL_FAVORITES_DESTROY, &params).await?;
        Ok(())
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
    async fn test_favorites() {
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
            .join("favorites.json");
        let mut testcase_file = std::fs::File::open(testcase_path).unwrap();
        let mut mock_response_body = String::new();
        testcase_file
            .read_to_string(&mut mock_response_body)
            .unwrap();
        let res = serde_json::from_str::<FavoritesResponse>(&mock_response_body).unwrap();
        let expect_posts = match res {
            FavoritesResponse::Succ { favorites } => favorites
                .into_iter()
                .map(|post| {
                    post.status.try_into().map_err(|e: Error| {
                        Error::DataConversionError(format!("post internal to post failed: {}", e))
                    })
                })
                .collect::<Result<Vec<Post>>>()
                .unwrap(),
            FavoritesResponse::Fail(_) => panic!("unexpected fail response"),
        };

        let mock_response = MockHttpResponse::new(200, &mock_response_body);
        mock_client.expect_get(URL_FAVORITES, mock_response);

        let posts = weibo_api.favorites(1).await.unwrap();
        assert_eq!(posts, expect_posts);
    }

    #[tokio::test]
    async fn test_favorites_destroy() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
        };
        let weibo_api = WeiboAPI::new(mock_client.clone(), session);
        let id = 12345;

        let mock_response = MockHttpResponse::new(200, "{}");
        mock_client.expect_post(URL_FAVORITES_DESTROY, mock_response);

        let result = weibo_api.favorites_destroy(id).await;
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod real_tests {
    use super::*;
    use crate::{client, session::Session, weibo_api::WeiboAPI};

    #[tokio::test]
    async fn test_real_favorites() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let client = client::new_client_with_headers().unwrap();
            let weibo_api = WeiboAPI::new(client, session);
            let _ = weibo_api.favorites(1).await.unwrap();
        }
    }
}
