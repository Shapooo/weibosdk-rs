#![allow(async_fn_in_trait)]
use crate::client::HttpClient;
use crate::constants::{params::*, urls::URL_FAVORITES};
use crate::utils;
use crate::weibo_api::WeiboAPI;
use anyhow::Result;
use serde::Serialize;

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

pub trait FavoritesAPI<C: HttpClient> {
    async fn favorites(&self, page: u32) -> Result<C::Response>;
    async fn favorites_create() -> ();
}

impl<C: HttpClient> FavoritesAPI<C> for WeiboAPI<C> {
    async fn favorites(&self, page: u32) -> Result<C::Response> {
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
            page: page,
            s: &s,
            source: SOURCE,
            ua: UA,
            wm: WM,
        };

        self.client.get(URL_FAVORITES, &params).await
    }

    async fn favorites_create() -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::HttpResponse,
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

        let mock_response = MockHttpResponse::new(200, "{\"ok\": 1}");
        mock_client.expect_get(URL_FAVORITES, mock_response);

        let res = weibo_api.favorites(1).await.unwrap();
        let json: serde_json::Value = res.json().await.unwrap();
        assert_eq!(json["ok"], 1);
    }
}
