use anyhow::Result;
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

pub trait HttpResponse: Send + Sync + 'static {
    async fn json<T: DeserializeOwned>(self) -> Result<T>;
    async fn text(self) -> Result<String>;
    async fn bytes(self) -> Result<Bytes>;
}

impl HttpResponse for reqwest::Response {
    async fn json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.json::<T>().await?)
    }

    async fn text(self) -> Result<String> {
        Ok(self.text().await?)
    }

    async fn bytes(self) -> Result<Bytes> {
        Ok(self.bytes().await?)
    }
}

pub trait HttpClient: Send + Sync + 'static {
    type Response: HttpResponse;
    async fn post(&self, url: &str, form: &(impl Serialize + Send + Sync)) -> Result<Self::Response>;
}

impl HttpClient for reqwest::Client {
    type Response = reqwest::Response;
    async fn post(&self, url: &str, form: &(impl Serialize + Send + Sync)) -> Result<Self::Response> {
        Ok(self.post(url).form(form).send().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct TestPayload {
        message: String,
    }

    #[tokio::test]
    async fn test_http_client_post() {
        let server = MockServer::start().await;
        let uri = format!("{}/test", server.uri());

        let expected_response = TestPayload {
            message: "success".to_string(),
        };

        Mock::given(method("POST"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let form = serde_json::json!({});
        let response = HttpClient::post(&client, &uri, &form).await.unwrap();

        let payload: TestPayload = response.json().await.unwrap();
        assert_eq!(payload, expected_response);
    }
}
