use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    client::{HttpClient, HttpResponse},
    config::Conifg,
    constants::{
        params::*,
        urls::{URL_LOGIN, URL_SEND_CODE},
    },
    err_response::ErrResponse,
    error::{Error, Result},
    session::Session,
};

#[derive(Debug, Clone)]
pub struct WeiboAPIImpl<C: HttpClient> {
    pub client: C,
    pub config: Conifg,
    login_state: LoginState,
}

#[derive(Debug, Clone, Default)]
pub enum LoginState {
    #[default]
    Init,
    WaitingForCode {
        phone_number: String,
    },
    LoggedIn {
        session: Session,
    },
}

impl LoginState {
    pub fn is_init(&self) -> bool {
        matches!(self, Self::Init)
    }

    pub fn is_waiting_for_code(&self) -> bool {
        matches!(self, Self::WaitingForCode { .. })
    }

    pub fn is_logged_in(&self) -> bool {
        matches!(self, Self::LoggedIn { .. })
    }
}

impl<C: HttpClient> WeiboAPIImpl<C> {
    pub fn new(client: C, config: Conifg) -> Self {
        info!("WeiboAPIImpl created");
        WeiboAPIImpl {
            client,
            config,
            login_state: Default::default(),
        }
    }

    pub fn login_state(&self) -> &LoginState {
        &self.login_state
    }

    #[cfg(any(feature = "test-mocks", test))]
    pub fn from_session(client: C, session: Session) -> Self {
        info!(
            "WeiboAPIImpl created from session for user {}",
            session.screen_name
        );
        WeiboAPIImpl {
            client,
            config: Default::default(),
            login_state: LoginState::LoggedIn { session },
        }
    }

    pub fn session(&self) -> Result<&Session> {
        if let LoginState::LoggedIn { ref session } = self.login_state {
            Ok(session)
        } else {
            warn!("session() called before login");
            Err(Error::NotLoggedIn)
        }
    }

    pub async fn get_sms_code(&mut self, phone_number: String) -> Result<()> {
        info!("getting sms code for phone number: {phone_number}");
        if !self.login_state.is_init() {
            warn!("get_sms_code called not in init state");
        }

        let payload = json!( {
            "c": PARAM_C,
            "from": FROM,
            "source": SOURCE,
            "lang": LANG,
            "locale": LOCALE,
            "wm": WM,
            "ua": UA,
            "phone": &phone_number,
        });
        let response = self
            .client
            .post(URL_SEND_CODE, &payload, self.config.retry_times)
            .await?;
        self.login_state = LoginState::WaitingForCode { phone_number };

        let send_code_response = response.json::<SendCodeResponse>().await?;
        match send_code_response {
            SendCodeResponse::Succ { msg } => {
                debug!("sms code sent successfully, get msg {msg}",);
                Ok(())
            }
            SendCodeResponse::Fail(err) => {
                error!("failed to get sms code: {err:?}");
                Err(Error::ApiError(err))
            }
        }
    }

    pub async fn login(&mut self, sms_code: &str) -> Result<()> {
        info!("logging in with sms code");
        if let LoginState::WaitingForCode { phone_number } = &self.login_state {
            let payload = json!({
                "c": PARAM_C,
                "lang": LANG,
                "getuser": "1",
                "getoauth": "1",
                "getcookie": "1",
                "phone": phone_number,
                "smscode": sms_code,
            });
            let session = execute_login(&self.client, &payload, self.config.retry_times).await?;
            info!("login success, user: {}", session.screen_name);
            self.login_state = LoginState::LoggedIn { session };
            Ok(())
        } else {
            error!("login called in invalid state");
            Err(Error::NotLoggedIn)
        }
    }

    pub async fn login_with_session(&mut self, session: Session) -> Result<()> {
        info!("logging in with session for user {}", session.screen_name);
        if let LoginState::Init = self.login_state {
            let payload = json!({
                "c": PARAM_C,
                "lang": LANG,
                "getuser": "1",
                "getoauth": "1",
                "getcookie": "1",
                "gsid": &session.gsid,
                "uid": &session.uid,
                "from": SESSION_REFRESH_FROM,
                "s": &crate::utils::generate_s(&session.uid, FROM),
            });
            let session = execute_login(&self.client, &payload, self.config.retry_times).await?;
            info!("login with session success, user: {}", session.screen_name);
            self.login_state = LoginState::LoggedIn { session };
            Ok(())
        } else {
            error!("login_with_session called in invalid state");
            Err(Error::NotLoggedIn)
        }
    }
}

impl<C: HttpClient> crate::WeiboAPI for WeiboAPIImpl<C> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::urls::{URL_LOGIN, URL_SEND_CODE};
    use crate::mock::{MockClient, MockHttpResponse};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_send_code() {
        let mock_client = MockClient::new();
        let phone_number = "1234567890".to_string();

        let send_code_response_json = json!({
            "msg": "success"
        });
        mock_client.expect_post(
            URL_SEND_CODE,
            MockHttpResponse::new(200, &send_code_response_json.to_string()),
        );

        let mut weibo_api = WeiboAPIImpl::new(mock_client.clone(), Default::default());
        weibo_api.get_sms_code(phone_number.clone()).await.unwrap();

        assert!(
            matches!(weibo_api.login_state, LoginState::WaitingForCode { phone_number: num } if num == phone_number)
        );
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
            URL_LOGIN,
            MockHttpResponse::new(200, &login_response_json.to_string()),
        );

        let mut weibo_api = WeiboAPIImpl {
            config: Default::default(),
            client: mock_client.clone(),
            login_state: LoginState::WaitingForCode {
                phone_number: phone_number.clone(),
            },
        };

        let result = weibo_api.login(&sms_code).await;

        assert!(result.is_ok());
        assert!(matches!(weibo_api.login_state, LoginState::LoggedIn { .. }));
        if let Ok(session) = weibo_api.session() {
            assert_eq!(session.gsid, "mock_gsid");
            assert_eq!(session.uid, "mock_uid");
            assert_eq!(session.screen_name, "mock_screen_name");
        } else {
            panic!("Login state should be LoggedIn");
        }
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
            URL_LOGIN,
            MockHttpResponse::new(200, &login_response_json.to_string()),
        );

        let mut weibo_api = WeiboAPIImpl::new(mock_client.clone(), Default::default());
        let result = weibo_api.login_with_session(old_session).await;

        assert!(result.is_ok());
        assert!(matches!(weibo_api.login_state, LoginState::LoggedIn { .. }));
        if let Ok(session) = weibo_api.session() {
            assert_eq!(session.gsid, "new_gsid");
        } else {
            panic!("Login state should be LoggedIn");
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum SendCodeResponse {
    Succ { msg: String },
    Fail(ErrResponse),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LoginResponse {
    Succ {
        gsid: String,
        uid: String,
        screen_name: String,
    },
    Fail(ErrResponse),
}

async fn execute_login<C: HttpClient, P: Serialize + Send + Sync>(
    client: &C,
    payload: &P,
    retry_times: u8,
) -> Result<Session> {
    let response = client.post(URL_LOGIN, payload, retry_times).await?;

    let response = response.json::<LoginResponse>().await?;

    match response {
        LoginResponse::Succ {
            gsid,
            uid,
            screen_name,
        } => Ok(Session {
            gsid,
            uid,
            screen_name,
        }),
        LoginResponse::Fail(err_res) => Err(Error::ApiError(err_res)),
    }
}
