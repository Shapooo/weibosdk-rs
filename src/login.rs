#![allow(async_fn_in_trait)]
use anyhow::Result;
use log::debug;
use reqwest::{
    Client,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::weibo_api::WeiboAPI;

const SEND_CODE_URL: &str = "https://api.weibo.cn/2/account/login_sendcode";
const LOGIN_URL: &str = "https://api.weibo.cn/2/account/login";

//-------------------------------------------------------------
//------------------------ Traits -----------------------------
//-------------------------------------------------------------

pub trait SendCodeAPI {
    type Login: LoginAPI;
    async fn get_send_code(self, phone_number: String) -> Result<Self::Login>;
}

pub trait LoginAPI {
    type WeiboClient;
    async fn login(self, sms_code: &str) -> Result<Self::WeiboClient>;
}

//-------------------------------------------------------------
//----------------------- SendCode ----------------------------
//-------------------------------------------------------------

#[derive(Debug, Serialize)]
struct SendCodePayload<'a> {
    c: &'a str,
    from: &'a str,
    source: &'a str,
    lang: &'a str,
    locale: &'a str,
    wm: &'a str,
    ua: &'a str,
    phone: &'a str,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SendCodeResponse {
    Succ {
        msg: String,
    },
    Fail {
        errmsg: String,
        errno: i32,
        errtype: String,
        isblock: bool,
    },
}

pub struct SendCode {
    client: Client,
}

impl SendCode {
    pub fn new() -> Self {
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
            (header::HOST, HeaderValue::from_static("api.weibo.cn")),
            (header::CONNECTION, HeaderValue::from_static("Keep-Alive")),
        ]);
        let client = Client::builder().default_headers(headers).build().unwrap();
        Self { client }
    }
}

impl SendCodeAPI for SendCode {
    type Login = WaitingLogin;
    async fn get_send_code(self, phone_number: String) -> Result<Self::Login> {
        let payload = SendCodePayload {
            c: "weicoabroad",
            from: "12DC195010",
            source: "4215535043",
            lang: "zh_CN",
            locale: "zh_CN",
            wm: "2468_1001",
            ua: "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710",
            phone: &phone_number,
        };
        let response: reqwest::Response = self
            .client
            .post(SEND_CODE_URL)
            .form(&payload)
            .send()
            .await?;

        let headers = response.headers();
        debug!("{:?}", headers);
        let send_code_response = response.json::<Value>().await.unwrap();
        debug!("{:?}", send_code_response);
        debug!(
            "{:?}",
            serde_json::from_value::<SendCodeResponse>(send_code_response).unwrap()
        );

        Ok(WaitingLogin {
            client: self.client,
            phone_number,
        })
    }
}

//-------------------------------------------------------------
//------------------------- Login------------------------------
//-------------------------------------------------------------

#[derive(Debug, Serialize)]
struct LoginPayload<'a> {
    c: &'a str,
    lang: &'a str,
    getuser: &'a str,
    getoauth: &'a str,
    getcookie: &'a str,
    phone: &'a str,
    smscode: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub gsid: String,
    pub uid: String,
    pub screen_name: String,
}

pub struct WaitingLogin {
    phone_number: String,
    client: Client,
}

impl LoginAPI for WaitingLogin {
    type WeiboClient = WeiboAPI;
    async fn login(self, sms_code: &str) -> Result<Self::WeiboClient> {
        let payload = LoginPayload {
            c: "weicoabroad",
            lang: "zh_CN",
            getuser: "1",
            getoauth: "1",
            getcookie: "1",
            phone: &self.phone_number,
            smscode: sms_code,
        };

        let response = self.client.post(LOGIN_URL).form(&payload).send().await?;

        let response = response.json::<LoginResponse>().await?;
        debug!("{:?}", response);

        Ok(WeiboAPI::new(
            self.client,
            response.gsid,
            response.uid,
            response.screen_name,
        ))
    }
}
