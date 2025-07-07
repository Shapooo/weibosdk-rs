use anyhow::Result;
use bytes::Bytes;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::client::{HttpClient, HttpResponse};

#[derive(Debug, Clone)]
pub struct MockHttpResponse {
    status: u16,
    body: String,
}

impl MockHttpResponse {
    pub fn new(status: u16, body: &str) -> Self {
        Self {
            status,
            body: body.to_string(),
        }
    }
}

impl HttpResponse for MockHttpResponse {
    async fn json<T: DeserializeOwned>(self) -> Result<T> {
        serde_json::from_str(&self.body).map_err(anyhow::Error::from)
    }

    async fn text(self) -> Result<String> {
        Ok(self.body)
    }

    async fn bytes(self) -> Result<Bytes> {
        Ok(Bytes::from(self.body))
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

    pub fn expect_post(&self, url: &str, response: MockHttpResponse) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(url.to_string(), response);
    }
}

impl HttpClient for MockClient {
    type Response = MockHttpResponse;

    async fn post(
        &self,
        url: &str,
        _form: &(impl Serialize + Send + Sync),
    ) -> Result<Self::Response> {
        let responses = self.responses.lock().unwrap();
        responses
            .get(url)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No mock response set for URL: {}", url))
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
        let response = mock_client.post(test_url, &form_data).await.unwrap();

        assert_eq!(response.status, 200);
        let received_data: TestData = response.json().await.unwrap();
        assert_eq!(received_data, expected_data);
    }

    #[tokio::test]
    async fn test_mock_client_post_no_response() {
        let mock_client = MockClient::new();
        let test_url = "http://example.com/api/test_fail";
        let form_data = serde_json::json!({ "key": "value" });

        let result = mock_client.post(test_url, &form_data).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No mock response set for URL: http://example.com/api/test_fail"
        );
    }
}
