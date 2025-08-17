#![allow(async_fn_in_trait)]
use std::sync::Arc;

use bytes::Bytes;
use log::{debug, info, trace};
use reqwest::{
    RequestBuilder,
    header::{self, HeaderMap, HeaderValue},
};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
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
    fn set_cookie(&mut self, cookie_store: CookieStore) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Client {
    main_client: reqwest::Client,
    web_client: Option<reqwest::Client>,
}

impl Client {
    pub fn new() -> Result<Self> {
        Ok(Self {
            main_client: make_main_client()?,
            web_client: None,
        })
    }

    pub fn main_client(&self) -> &reqwest::Client {
        &self.main_client
    }

    pub fn web_client(&self) -> Option<&reqwest::Client> {
        self.web_client.as_ref()
    }

    async fn send_request(
        &self,
        request_builder: RequestBuilder,
        retry_times: u8,
    ) -> Result<reqwest::Response> {
        let mut attempts = 0;
        loop {
            let result = request_builder.try_clone().unwrap().send().await;
            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response);
                    } else {
                        return Err(Error::NetworkError(
                            response.error_for_status().err().unwrap(),
                        ));
                    }
                }
                Err(e) => {
                    if e.is_timeout() && attempts < retry_times {
                        attempts += 1;
                        continue;
                    }
                    return Err(e.into());
                }
            }
        }
    }
}

fn make_main_client() -> Result<reqwest::Client> {
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

fn make_web_client(cookie_store: CookieStore) -> Result<reqwest::Client> {
    info!("Creating new http client with default headers");
    let headers = HeaderMap::from_iter([
        (
            header::USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:141.0) Gecko/20100101 Firefox/141.0",
            ),
        ),
        (header::ACCEPT_ENCODING, HeaderValue::from_static("gzip")),
        (
            header::ACCEPT,
            HeaderValue::from_static("application/json, text/plain, */*"),
        ),
        (
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_static("en-US,en;q=0.5"),
        ),
        (
            header::REFERER,
            HeaderValue::from_static("https://weibo.com/"),
        ),
        (header::TE, HeaderValue::from_static("trailers")),
    ]);
    Ok(reqwest::Client::builder()
        .default_headers(headers)
        .cookie_provider(Arc::new(CookieStoreMutex::new(cookie_store)))
        .build()?)
}

impl HttpClient for Client {
    type Response = reqwest::Response;
    async fn get(
        &self,
        url: &str,
        query: &(impl Serialize + Send + Sync),
        retry_times: u8,
    ) -> Result<Self::Response> {
        debug!("Sending GET request to {url}");
        trace!(
            "GET request query: {}",
            serde_json::to_string_pretty(query).unwrap_or_default()
        );
        let url = url::Url::parse(url).map_err(|e| Error::DataConversionError(format!("{e}")))?;
        let client = if url.domain() == Some("weibo.com") {
            self.web_client.as_ref().ok_or(Error::NotLoggedIn)?
        } else {
            &self.main_client
        };

        let request_builder = client.get(url).query(query);
        self.send_request(request_builder, retry_times).await
    }

    async fn post(
        &self,
        url: &str,
        form: &(impl Serialize + Send + Sync),
        retry_times: u8,
    ) -> Result<Self::Response> {
        debug!("Sending POST request to {url}");
        trace!(
            "POST request form: {}",
            serde_json::to_string_pretty(form).unwrap_or_default()
        );
        let url = url::Url::parse(url).map_err(|e| Error::DataConversionError(format!("{e}")))?;
        let client = if url.domain() == Some("weibo.com") {
            self.web_client.as_ref().ok_or(Error::NotLoggedIn)?
        } else {
            &self.main_client
        };
        let request_builder = client.post(url).form(form);
        self.send_request(request_builder, retry_times).await
    }

    fn set_cookie(&mut self, cookie_store: CookieStore) -> Result<()> {
        self.web_client = Some(make_web_client(cookie_store)?);
        Ok(())
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

        let client = Client::new().unwrap();
        let form = serde_json::json!({});
        let response = HttpClient::post(&client, &uri, &form, 3).await.unwrap();

        let payload: TestPayload = response.json().await.unwrap();
        assert_eq!(payload, expected_response);
    }
}
