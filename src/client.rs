#![allow(async_fn_in_trait)]
use bytes::Bytes;
use log::{debug, error, info, trace};
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::{Error, Result};

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

pub trait HttpClient: Send + Sync + Clone + 'static {
    type Response: HttpResponse;
    async fn get(
        &self,
        url: &str,
        query: &(impl Serialize + Send + Sync),
        retry_times: u8,
    ) -> Result<Self::Response>;
    async fn post(
        &self,
        url: &str,
        form: &(impl Serialize + Send + Sync),
        retry_times: u8,
    ) -> Result<Self::Response>;
}

pub fn new_client_with_headers() -> Result<reqwest::Client> {
    info!("Creating new http client with default headers");
    let headers = HeaderMap::from_iter([
        (
            header::USER_AGENT,
            HeaderValue::from_static("HONOR-PGT-AN10_9_WeiboIntlAndroid_6710"),
        ),
        (header::ACCEPT_ENCODING, HeaderValue::from_static("gzip")),
        (
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
        ),
        (header::CONNECTION, HeaderValue::from_static("Keep-Alive")),
    ]);
    Ok(reqwest::Client::builder()
        .default_headers(headers)
        .build()?)
}

impl HttpClient for reqwest::Client {
    type Response = reqwest::Response;
    async fn get(
        &self,
        url: &str,
        query: &(impl Serialize + Send + Sync),
        retry_times: u8,
    ) -> Result<Self::Response> {
        let mut attempts = 0;
        debug!("Sending GET request to {url}");
        trace!(
            "GET request query: {}",
            serde_json::to_string_pretty(query).unwrap_or_default()
        );
        loop {
            let result = self.get(url).query(query).send().await;
            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        debug!("GET request to {url} success");
                        return Ok(response);
                    } else {
                        error!(
                            "GET request to {} failed with status {}",
                            url,
                            response.status()
                        );
                        return Err(Error::NetworkError(
                            response.error_for_status().err().unwrap(),
                        ));
                    }
                }
                Err(e) => {
                    error!("GET request to {url} failed with error: {e}");
                    if e.is_timeout() && attempts < retry_times {
                        attempts += 1;
                        debug!("Retrying GET request to {url}, attempt {attempts}");
                        continue;
                    }
                    return Err(e.into());
                }
            }
        }
    }

    async fn post(
        &self,
        url: &str,
        form: &(impl Serialize + Send + Sync),
        retry_times: u8,
    ) -> Result<Self::Response> {
        let mut attempts = 0;
        debug!("Sending POST request to {url}");
        trace!(
            "POST request form: {}",
            serde_json::to_string_pretty(form).unwrap_or_default()
        );
        loop {
            let result = self.post(url).form(form).send().await;
            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        debug!("POST request to {url} success");
                        return Ok(response);
                    } else {
                        error!(
                            "POST request to {} failed with status {}",
                            url,
                            response.status()
                        );
                        return Err(Error::NetworkError(
                            response.error_for_status().err().unwrap(),
                        ));
                    }
                }
                Err(e) => {
                    error!("POST request to {url} failed with error: {e}");
                    if e.is_timeout() && attempts < retry_times {
                        attempts += 1;
                        debug!("Retrying POST request to {url}, attempt {attempts}");
                        continue;
                    }
                    return Err(e.into());
                }
            }
        }
    }
}

#[cfg(test)]
mod local_tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
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
        let response = HttpClient::post(&client, &uri, &form, 3).await.unwrap();

        let payload: TestPayload = response.json().await.unwrap();
        assert_eq!(payload, expected_response);
    }
}
