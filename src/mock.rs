use bytes::Bytes;
use serde::{Serialize, de::DeserializeOwned};

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::constants::urls::*;
use crate::error::{Error, Result};
use crate::http_client::{HttpClient, HttpResponse};

#[derive(Debug, Clone)]
pub struct MockHttpResponse {
    #[allow(unused)]
    status: u16,
    body: Bytes,
}

impl MockHttpResponse {
    pub fn new(status: u16, body: &str) -> Self {
        Self {
            status,
            body: Bytes::from(body.to_string()),
        }
    }

    pub fn new_with_bytes(status: u16, body: Bytes) -> Self {
        Self { status, body }
    }
}

impl HttpResponse for MockHttpResponse {
    async fn json<T: DeserializeOwned>(self) -> Result<T> {
        serde_json::from_slice(&self.body).map_err(Error::from)
    }

    async fn text(self) -> Result<String> {
        String::from_utf8(self.body.to_vec()).map_err(|e| Error::DataConversionError(e.to_string()))
    }

    async fn bytes(self) -> Result<Bytes> {
        Ok(self.body)
    }
}

#[derive(Clone, Debug, Default)]
pub struct MockClient {
    responses: Arc<Mutex<HashMap<String, MockHttpResponse>>>,
}

impl MockClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn expect_get(&self, url: &str, response: MockHttpResponse) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(url.to_string(), response);
    }

    pub fn expect_post(&self, url: &str, response: MockHttpResponse) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(url.to_string(), response);
    }

    fn _expect_get_from_str(&self, url: &str, content: &str) {
        self.expect_get(url, MockHttpResponse::new(200, content));
    }

    fn _expect_get_from_file(&self, url: &str, path: &Path) -> std::io::Result<()> {
        let content = fs::read_to_string(path)?;
        self._expect_get_from_str(url, &content);
        Ok(())
    }

    fn _expect_post_from_str(&self, url: &str, content: &str) {
        self.expect_post(url, MockHttpResponse::new(200, content));
    }

    fn _expect_post_from_file(&self, url: &str, path: &Path) -> std::io::Result<()> {
        let content = fs::read_to_string(path)?;
        self._expect_post_from_str(url, &content);
        Ok(())
    }

    pub fn set_favorites_response_from_str(&self, content: &str) {
        self._expect_get_from_str(URL_FAVORITES, content)
    }

    pub fn set_favorites_response_from_file(&self, path: &Path) -> std::io::Result<()> {
        self._expect_get_from_file(URL_FAVORITES, path)
    }

    pub fn set_profile_statuses_response_from_str(&self, content: &str) {
        self._expect_get_from_str(URL_PROFILE_STATUSES, content)
    }

    pub fn set_profile_statuses_response_from_file(&self, path: &Path) -> std::io::Result<()> {
        self._expect_get_from_file(URL_PROFILE_STATUSES, path)
    }

    pub fn set_favorites_destroy_response_from_str(&self, content: &str) {
        self._expect_post_from_str(URL_FAVORITES_DESTROY, content)
    }

    pub fn set_favorites_destroy_response_from_file(&self, path: &Path) -> std::io::Result<()> {
        self._expect_post_from_file(URL_FAVORITES_DESTROY, path)
    }

    pub fn set_get_sms_code_response_from_str(&self, content: &str) {
        self._expect_post_from_str(URL_SEND_CODE, content)
    }

    pub fn set_get_sms_code_response_from_file(&self, path: &Path) -> std::io::Result<()> {
        self._expect_post_from_file(URL_SEND_CODE, path)
    }

    pub fn set_login_response_from_str(&self, content: &str) {
        self._expect_post_from_str(URL_LOGIN, content)
    }

    pub fn set_login_response_from_file(&self, path: &Path) -> std::io::Result<()> {
        self._expect_post_from_file(URL_LOGIN, path)
    }

    pub fn set_statuses_show_response_from_str(&self, content: &str) {
        self._expect_get_from_str(URL_STATUSES_SHOW, content)
    }

    pub fn set_statuses_show_response_from_file(&self, path: &Path) -> std::io::Result<()> {
        self._expect_get_from_file(URL_STATUSES_SHOW, path)
    }

    pub fn set_emoji_update_response_from_str(&self, content: &str) {
        self._expect_post_from_str(URL_EMOJI_UPDATE, content)
    }

    pub fn set_emoji_update_response_from_file(&self, path: &Path) -> std::io::Result<()> {
        self._expect_post_from_file(URL_EMOJI_UPDATE, path)
    }

    pub fn set_web_emoticon_response_from_str(&self, content: &str) {
        self._expect_get_from_str(URL_WEB_EMOTICON, content);
    }

    pub fn set_web_emoticon_response_from_file(&self, path: &Path) -> std::io::Result<()> {
        self._expect_get_from_file(URL_WEB_EMOTICON, path)
    }
}

impl HttpClient for MockClient {
    type Response = MockHttpResponse;

    async fn get(
        &self,
        url: &str,
        _query: &(impl Serialize + Send + Sync),
        _retry_times: u8,
    ) -> Result<Self::Response> {
        let responses = self.responses.lock().unwrap();
        responses.get(url).cloned().ok_or_else(|| {
            Error::DataConversionError(format!("No mock response set for URL: {url}"))
        })
    }

    async fn post(
        &self,
        url: &str,
        _form: &(impl Serialize + Send + Sync),
        _retry_times: u8,
    ) -> Result<Self::Response> {
        let responses = self.responses.lock().unwrap();
        responses.get(url).cloned().ok_or_else(|| {
            Error::DataConversionError(format!("No mock response set for URL: {url}"))
        })
    }

    fn set_cookie(&self, _cookie_store: reqwest_cookie_store::CookieStore) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod local_tests {
    use super::*;
    use serde::Deserialize;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct TestData {
        value: String,
    }

    #[tokio::test]
    async fn test_mock_client_post() {
        let mock_client = MockClient::new();
        let test_url = "http://example.com/api/test";
        let expected_data = TestData {
            value: "mocked_response".to_string(),
        };
        let expected_json = serde_json::to_string(&expected_data).unwrap();

        mock_client.expect_post(test_url, MockHttpResponse::new(200, &expected_json));

        let form_data = serde_json::json!({ "key": "value" });
        let response = mock_client.post(test_url, &form_data, 2).await.unwrap();

        assert_eq!(response.status, 200);
        let received_data: TestData = response.json().await.unwrap();
        assert_eq!(received_data, expected_data);
    }

    #[tokio::test]
    async fn test_mock_client_post_no_response() {
        let mock_client = MockClient::new();
        let test_url = "http://example.com/api/test_fail";
        let form_data = serde_json::json!({ "key": "value" });

        let result = mock_client.post(test_url, &form_data, 2).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to convert data: No mock response set for URL: http://example.com/api/test_fail"
        );
    }

    macro_rules! test_setter {
        ($test_name:ident, $method_str:ident, $method_file:ident, $url:expr, $is_get:expr) => {
            #[tokio::test]
            async fn $test_name() {
                let client = MockClient::new();
                let expected_body = format!("{{\"name\": \"{}\"}}", stringify!($test_name));

                // Test from_str
                client.$method_str(&expected_body);
                let resp = if $is_get {
                    client.get($url, &(), 0).await.unwrap()
                } else {
                    client.post($url, &(), 0).await.unwrap()
                };
                let body = resp.text().await.unwrap();
                assert_eq!(body, expected_body);

                // Test from_file
                let mut temp_file = NamedTempFile::new().unwrap();
                write!(temp_file, "{}", &expected_body).unwrap();
                client.$method_file(temp_file.path()).unwrap();
                let resp = if $is_get {
                    client.get($url, &(), 0).await.unwrap()
                } else {
                    client.post($url, &(), 0).await.unwrap()
                };
                let body = resp.text().await.unwrap();
                assert_eq!(body, expected_body);
            }
        };
    }

    test_setter!(
        test_set_favorites,
        set_favorites_response_from_str,
        set_favorites_response_from_file,
        URL_FAVORITES,
        true
    );

    test_setter!(
        test_set_profile_statuses,
        set_profile_statuses_response_from_str,
        set_profile_statuses_response_from_file,
        URL_PROFILE_STATUSES,
        true
    );

    test_setter!(
        test_set_favorites_destroy,
        set_favorites_destroy_response_from_str,
        set_favorites_destroy_response_from_file,
        URL_FAVORITES_DESTROY,
        false
    );

    test_setter!(
        test_set_get_sms_code,
        set_get_sms_code_response_from_str,
        set_get_sms_code_response_from_file,
        URL_SEND_CODE,
        false
    );

    test_setter!(
        test_set_login,
        set_login_response_from_str,
        set_login_response_from_file,
        URL_LOGIN,
        false
    );

    test_setter!(
        test_set_statuses_show,
        set_statuses_show_response_from_str,
        set_statuses_show_response_from_file,
        URL_STATUSES_SHOW,
        true
    );

    test_setter!(
        test_set_emoji_update,
        set_emoji_update_response_from_str,
        set_emoji_update_response_from_file,
        URL_EMOJI_UPDATE,
        false
    );
}
