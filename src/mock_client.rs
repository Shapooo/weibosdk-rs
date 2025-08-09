use bytes::Bytes;
use serde::{Serialize, de::DeserializeOwned};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::client::{HttpClient, HttpResponse};
use crate::err_response::ErrResponse;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct MockHttpResponse {
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

    #[allow(unused)]
    pub fn new_with_bytes(status: u16, body: &[u8]) -> Self {
        Self {
            status,
            body: Bytes::from(body.to_vec()),
        }
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

#[derive(Clone, Debug)]
pub struct MockClient {
    responses: Arc<Mutex<HashMap<String, MockHttpResponse>>>,
}

impl MockClient {
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn expect_get(&self, url: &str, response: MockHttpResponse) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(url.to_string(), response);
    }

    pub fn expect_post(&self, url: &str, response: MockHttpResponse) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(url.to_string(), response);
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
            Error::ApiError(ErrResponse {
                errmsg: format!("No mock response set for URL: {}", url),
                ..Default::default()
            })
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
            Error::ApiError(ErrResponse {
                errmsg: format!("No mock response set for URL: {}", url),
                ..Default::default()
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

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
}
