#![allow(async_fn_in_trait)]
use log::{debug, error, info};
use serde::Deserialize;

use crate::{
    Post,
    client::{HttpClient, HttpResponse},
    constants::{
        params::*,
        urls::{URL_FAVORITES, URL_FAVORITES_DESTROY},
    },
    error::{Error, Result},
    models::err_response::ErrResponse,
    utils,
    weibo_api::WeiboAPIImpl,
};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FavoritesPost {
    pub status: Post,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum FavoritesResponse {
    Succ(FavoritesSucc),
    Fail(ErrResponse),
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FavoritesSucc {
    pub favorites: Vec<FavoritesPost>,
    #[serde(default)]
    #[allow(unused)]
    pub total_number: i32,
}

impl TryFrom<FavoritesResponse> for Vec<Post> {
    type Error = Error;
    fn try_from(value: FavoritesResponse) -> Result<Self> {
        let res = value;
        match res {
            FavoritesResponse::Succ(FavoritesSucc { favorites, .. }) => {
                debug!("got {} favorites", favorites.len());
                let posts = favorites
                    .into_iter()
                    .map(|post| post.status)
                    .collect::<Vec<Post>>();
                Ok(posts)
            }
            FavoritesResponse::Fail(err) => {
                error!("failed to get favorites: {err:?}");
                Err(Error::ApiError(err))
            }
        }
    }
}

impl From<FavoritesSucc> for Vec<Post> {
    fn from(value: FavoritesSucc) -> Self {
        value.favorites.into_iter().map(|p| p.status).collect()
    }
}

pub trait FavoritesAPI {
    async fn favorites(&self, page: u32) -> Result<Vec<Post>>;
    async fn favorites_destroy(&self, id: i64) -> Result<()>;
}

impl<C: HttpClient> FavoritesAPI for WeiboAPIImpl<C> {
    async fn favorites(&self, page: u32) -> Result<Vec<Post>> {
        info!("getting favorites, page: {page}");
        let session = self.session()?;
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["page"] = page.into();
        params["count"] = self.config.fav_count.into();
        params["mix_media_enable"] = MIX_MEDIA_ENABLE.into();

        let response = self
            .client
            .get(URL_FAVORITES, &params, self.config.retry_times)
            .await?;
        response.json::<FavoritesResponse>().await?.try_into()
    }

    async fn favorites_destroy(&self, id: i64) -> Result<()> {
        info!("destroying favorite, id: {id}");
        let session = self.session()?;
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["id"] = id.into();
        let _ = self
            .client
            .post(URL_FAVORITES_DESTROY, &params, self.config.retry_times)
            .await?;
        debug!("favorite {id} destroyed");
        Ok(())
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
    async fn test_favorites() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
            cookie_store: Default::default(),
        };
        let weibo_api = WeiboAPIImpl::from_session(mock_client.clone(), session);

        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let testcase_path = manifest_dir.join("tests/data/favorites.json");
        let mock_response_body = std::fs::read_to_string(testcase_path).unwrap();

        let mock_response = MockHttpResponse::new(200, &mock_response_body);
        mock_client.expect_get(URL_FAVORITES, mock_response);

        weibo_api.favorites(1).await.unwrap();
    }

    #[tokio::test]
    async fn test_favorites_destroy() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
            cookie_store: Default::default(),
        };
        let weibo_api = WeiboAPIImpl::from_session(mock_client.clone(), session);
        let id = 12345;

        let mock_response = MockHttpResponse::new(200, "{}");
        mock_client.expect_post(URL_FAVORITES_DESTROY, mock_response);

        weibo_api.favorites_destroy(id).await.unwrap();
    }
}

#[cfg(test)]
mod real_tests {
    use super::*;
    use crate::{client, session::Session, weibo_api::WeiboAPIImpl};

    #[tokio::test]
    async fn test_real_favorites() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let client = client::Client::new().unwrap();
            let weibo_api = WeiboAPIImpl::from_session(client, session);
            let _ = weibo_api.favorites(1).await.unwrap();
        }
    }
}
