#![allow(async_fn_in_trait)]
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Post;
use crate::client::{HttpClient, HttpResponse};
use crate::constants::{params::*, urls::URL_FAVORITES};
use crate::internal::post::PostInternal;
use crate::utils;
use crate::weibo_api::WeiboAPI;

#[derive(Serialize, Debug)]
struct FavoritesParams<'a> {
    c: &'a str,
    count: u8,
    from: &'a str,
    gsid: &'a str,
    lang: &'a str,
    locale: &'a str,
    mix_media_enable: u8,
    page: u32,
    s: &'a str,
    source: &'a str,
    ua: &'a str,
    wm: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
struct FavoritesPost {
    pub status: PostInternal,
}

#[derive(Debug, Clone, Deserialize)]
struct FavoritesResponse {
    pub favorites: Vec<FavoritesPost>,
}

pub trait FavoritesAPI<C: HttpClient> {
    async fn favorites(&self, page: u32) -> Result<Vec<Post>>;
    async fn favorites_create() -> ();
}

impl<C: HttpClient> FavoritesAPI<C> for WeiboAPI<C> {
    async fn favorites(&self, page: u32) -> Result<Vec<Post>> {
        let session = self.session();
        let s = utils::generate_s(&session.uid, FROM);
        let params = FavoritesParams {
            c: PARAM_C,
            count: COUNT,
            from: FROM,
            gsid: &session.gsid,
            lang: LANG,
            locale: LOCALE,
            mix_media_enable: MIX_MEDIA_ENABLE,
            page,
            s: &s,
            source: SOURCE,
            ua: UA,
            wm: WM,
        };

        let response = self.client.get(URL_FAVORITES, &params).await?;
        let res = response.json::<FavoritesResponse>().await?;
        let posts = res
            .favorites
            .into_iter()
            .map(|post| post.status.try_into())
            .collect::<Result<Vec<Post>>>()?;
        Ok(posts)
    }

    async fn favorites_create() -> () {
        todo!()
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
        let expect_posts = serde_json::from_str::<FavoritesResponse>(&mock_response_body)
            .unwrap()
            .favorites
            .into_iter()
            .map(|post| post.status.try_into())
            .collect::<Result<Vec<Post>>>()
            .unwrap();
        let mock_response = MockHttpResponse::new(200, &mock_response_body);
        mock_client.expect_get(URL_FAVORITES, mock_response);

        let posts = weibo_api.favorites(1).await.unwrap();
        assert_eq!(posts, expect_posts);
    }
}
