#![allow(async_fn_in_trait)]
use anyhow::Result;
use bytes::Bytes;

use crate::client::{HttpClient, HttpResponse};
use crate::weibo_api::WeiboAPI;

pub trait PictureDownloader {
    async fn download_picture(&self, url: &str) -> Result<Bytes>;
}

impl<C: HttpClient> PictureDownloader for WeiboAPI<C> {
    async fn download_picture(&self, url: &str) -> Result<Bytes> {
        let client = &self.client;
        let response = client.get(url, &serde_json::json!({})).await?;
        let bytes = response.bytes().await?;
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mock_client::{MockClient, MockHttpResponse},
        session::Session,
    };

    #[tokio::test]
    async fn test_download_picture() {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "test_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
        };
        let weibo_api = WeiboAPI::new(mock_client.clone(), session);

        let picture_data = b"This is a fake picture file.";
        let url = "http://example.com/picture.jpg";

        let mock_response = MockHttpResponse::new_with_bytes(200, picture_data);
        mock_client.expect_get(url, mock_response);

        let result = weibo_api.download_picture(url).await.unwrap();

        assert_eq!(result, Bytes::from_static(picture_data));
    }
}
