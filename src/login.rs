#![allow(async_fn_in_trait)]
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::{HttpClient, HttpResponse};
use crate::error::LoginError;
use crate::session::Session;

const SEND_CODE_URL: &str = "https://api.weibo.cn/2/account/login_sendcode";
const LOGIN_URL: &str = "https://api.weibo.cn/2/account/login";
const FROM1: &str = "12DC195010";
const LOGIN_FROM: &str = "1299295010";

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

pub struct SendCode<C: HttpClient> {
    client: C,
}

impl<C: HttpClient> SendCode<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &C {
        &self.client
    }

    pub async fn get_send_code(self, phone_number: String) -> Result<WaitingLogin<C>> {
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
        let response = self.client.post(SEND_CODE_URL, &payload).await?;

        let send_code_response = response.json::<Value>().await?;
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

#[derive(Debug, Serialize)]
struct LoginWithGsidPayload<'a> {
    c: &'a str,
    lang: &'a str,
    getuser: &'a str,
    getoauth: &'a str,
    getcookie: &'a str,
    from: &'a str,
    gsid: &'a str,
    uid: &'a str,
    s: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub gsid: String,
    pub uid: String,
    pub screen_name: String,
}

pub struct WaitingLogin<C: HttpClient> {
    phone_number: String,
    client: C,
}

impl<C: HttpClient> WaitingLogin<C> {
    pub async fn login(self, sms_code: &str) -> std::result::Result<Session, LoginError> {
        let payload = LoginPayload {
            c: "weicoabroad",
            lang: "zh_CN",
            getuser: "1",
            getoauth: "1",
            getcookie: "1",
            phone: &self.phone_number,
            smscode: sms_code,
        };

        let response = self
            .client
            .post(LOGIN_URL, &payload)
            .await
            .map_err(|e| LoginError::NetworkError(e.into()))?;

        let response = response
            .json::<LoginResponse>()
            .await
            .map_err(|e| LoginError::NetworkError(e.into()))?;
        debug!("{:?}", response);

        Ok(Session {
            gsid: response.gsid,
            uid: response.uid,
            screen_name: response.screen_name,
        })
    }
}

pub struct Login<C: HttpClient> {
    client: C,
}

impl<C: HttpClient> Login<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub async fn login_with_session(
        &self,
        session: Session,
    ) -> std::result::Result<Session, LoginError> {
        let payload = LoginWithGsidPayload {
            c: "weicoabroad",
            lang: "zh_CN",
            getuser: "1",
            getoauth: "1",
            getcookie: "1",
            gsid: &session.gsid,
            uid: &session.uid,
            from: LOGIN_FROM,
            s: &crate::utils::generate_s(&session.uid, LOGIN_FROM),
        };

        let response = self
            .client
            .post(LOGIN_URL, &payload)
            .await
            .map_err(|e| LoginError::NetworkError(e.into()))?;

        let response = response
            .json::<LoginResponse>()
            .await
            .map_err(|e| LoginError::NetworkError(e.into()))?;
        debug!("{:?}", response);

        Ok(Session {
            gsid: response.gsid,
            uid: response.uid,
            screen_name: response.screen_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_client::{MockClient, MockHttpResponse};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_send_code() {
        let mock_client = MockClient::new();
        let phone_number = "1234567890".to_string();

        let send_code_response_json = json!({
            "msg": "success"
        });
        mock_client.expect_post(
            SEND_CODE_URL,
            MockHttpResponse::new(200, &send_code_response_json.to_string()),
        );

        let send_code_instance = SendCode::new(mock_client.clone());
        let waiting_login_result = send_code_instance.get_send_code(phone_number.clone()).await;

        assert!(waiting_login_result.is_ok());
        let waiting_login = waiting_login_result.unwrap();
        assert_eq!(waiting_login.phone_number, phone_number);
    }

    #[tokio::test]
    async fn test_login() {
        let mock_client = MockClient::new();
        let phone_number = "1234567890".to_string();
        let sms_code = "123456".to_string();

        let login_response_json = json!({
            "gsid": "mock_gsid",
            "uid": "mock_uid",
            "screen_name": "mock_screen_name"
        });
        mock_client.expect_post(
            LOGIN_URL,
            MockHttpResponse::new(200, &login_response_json.to_string()),
        );

        let waiting_login_instance = WaitingLogin {
            phone_number: phone_number.clone(),
            client: mock_client.clone(),
        };

        let session_result = waiting_login_instance.login(&sms_code).await;

        assert!(session_result.is_ok());
        let session = session_result.unwrap();
        assert_eq!(session.gsid, "mock_gsid");
        assert_eq!(session.uid, "mock_uid");
        assert_eq!(session.screen_name, "mock_screen_name");
    }

    #[tokio::test]
    async fn test_login_with_session() {
        let mock_client = MockClient::new();
        let old_session = Session {
            gsid: "old_gsid".to_string(),
            uid: "test_uid".to_string(),
            screen_name: "test_screen_name".to_string(),
        };

        let login_response_json = json!({
            "gsid": "new_gsid",
            "uid": "test_uid",
            "screen_name": "test_screen_name"
        });
        mock_client.expect_post(
            LOGIN_URL,
            MockHttpResponse::new(200, &login_response_json.to_string()),
        );

        let login_client = Login::new(mock_client.clone());
        let new_session_result = login_client.login_with_session(old_session).await;

        assert!(new_session_result.is_ok());
        let new_session = new_session_result.unwrap();
        assert_eq!(new_session.gsid, "new_gsid");
    }
}
