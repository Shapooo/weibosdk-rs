#![allow(async_fn_in_trait)]
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


use super::client::WeiboClient;
use super::internal::User;

const SEND_CODE_URL: &str = "https://api.weibo.cn/2/account/login_sendcode";
const LOGIN_URL: &str = "https://api.weibo.cn/2/account/login";

///////////////////////////////////////////////////////////////
////////////////////////// Traits /////////////////////////////
///////////////////////////////////////////////////////////////

pub trait SendCodeAPI {
    type Login: LoginAPI;
    async fn get_send_code(self, pn: &str) -> Result<Self::Login>;
}

pub trait LoginAPI {
    type WeiboClient;
    async fn login(self, pn: &str, sms_code: &str) -> Result<Self::WeiboClient>;
}

///////////////////////////////////////////////////////////////
///////////////////////// SendCode ////////////////////////////
///////////////////////////////////////////////////////////////

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

#[derive(Debug, Deserialize)]
pub struct SendCodeResponse {
    pub msg: String,
    pub code: String,
}

pub struct SendCode {
    client: Client,
}

impl SendCode {
    pub fn new() -> Self {
        SendCode {
            client: Client::new(),
        }
    }
}

impl SendCodeAPI for SendCode {
    type Login = WaitingLogin;
    async fn get_send_code(self, phone_number: &str) -> Result<Self::Login> {
        let payload = SendCodePayload {
            c: "weicoabroad",
            from: "12DC195010",
            source: "4215535043",
            lang: "zh_CN",
            locale: "zh_CN",
            wm: "2468_1001",
            ua: "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710",
            phone: phone_number,
        };

        let response = self
            .client
            .post(SEND_CODE_URL)
            .header("User-Agent", "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710")
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .header("Host", "api.weibo.cn")
            .header("Connection", "Keep-Alive")
            .header("Accept-Encoding", "gzip")
            .form(&payload)
            .send()
            .await?;

        let send_code_response = response.json::<SendCodeResponse>().await?;

        Ok(WaitingLogin {
            client: self.client,
        })
    }
}

///////////////////////////////////////////////////////////////
/////////////////////////// Login /////////////////////////////
///////////////////////////////////////////////////////////////

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
    pub user: User,
}

pub struct WaitingLogin {
    client: Client,
}

impl LoginAPI for WaitingLogin {
    type WeiboClient = WeiboClient;
    async fn login(self, phone_number: &str, sms_code: &str) -> Result<Self::WeiboClient> {
        let payload = LoginPayload {
            c: "weicoabroad",
            lang: "zh_CN",
            getuser: "1",
            getoauth: "1",
            getcookie: "1",
            phone: phone_number,
            smscode: sms_code,
        };

        let response = self
            .client
            .post(LOGIN_URL)
            .header("User-Agent", "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710")
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .header("Host", "api.weibo.cn")
            .header("Connection", "Keep-Alive")
            .header("Accept-Encoding", "gzip")
            .form(&payload)
            .send()
            .await?
            .json::<LoginResponse>()
            .await?;

        Ok(WeiboClient::new(self.client, login_response))
    }
}
