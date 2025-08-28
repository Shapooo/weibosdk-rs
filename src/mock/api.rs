use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{
    WeiboAPI,
    config::Conifg,
    emoji::EmojiUpdateAPI,
    error::Result,
    favorites::FavoritesAPI,
    models::post::Post,
    profile_statuses::ProfileStatusesAPI,
    session::Session,
    statuses_show::StatusesShowAPI,
    weibo_api::{LoginState, WeiboAPIImpl},
};

use super::client::MockClient;

#[derive(Clone)]
pub struct MockAPI {
    client: WeiboAPIImpl<MockClient>,
}

impl MockAPI {
    pub fn new(client: MockClient) -> Self {
        Self {
            client: WeiboAPIImpl::new(client, Conifg::default()),
        }
    }

    pub fn from_session(client: MockClient, session: Arc<Mutex<Session>>) -> Self {
        Self {
            client: WeiboAPIImpl::from_session(client, session),
        }
    }

    pub fn login_state(&self) -> &LoginState {
        self.client.login_state()
    }

    pub async fn get_sms_code(&mut self, phone_number: String) -> Result<()> {
        self.client.get_sms_code(phone_number).await
    }

    pub async fn login(&mut self, sms_code: &str) -> Result<()> {
        self.client.login(sms_code).await
    }

    pub async fn login_with_session(&mut self, session: Arc<Mutex<Session>>) -> Result<()> {
        self.client.login_with_session(session).await
    }
}

impl EmojiUpdateAPI for MockAPI {
    async fn emoji_update(&self) -> Result<HashMap<String, String>> {
        self.client.emoji_update().await
    }
}

impl FavoritesAPI for MockAPI {
    async fn favorites(&self, page: u32) -> Result<Vec<Post>> {
        self.client.favorites(page).await
    }

    async fn favorites_destroy(&self, id: i64) -> Result<()> {
        self.client.favorites_destroy(id).await
    }
}

impl StatusesShowAPI for MockAPI {
    async fn statuses_show(&self, id: i64) -> Result<Post> {
        self.client.statuses_show(id).await
    }
}

impl ProfileStatusesAPI for MockAPI {
    async fn profile_statuses(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        self.client.profile_statuses(uid, page).await
    }

    async fn profile_statuses_original(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        self.client.profile_statuses_original(uid, page).await
    }

    async fn profile_statuses_picture(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        self.client.profile_statuses_picture(uid, page).await
    }

    async fn profile_statuses_video(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        self.client.profile_statuses_video(uid, page).await
    }

    async fn profile_statuses_article(&self, uid: i64, page: u32) -> Result<Vec<Post>> {
        self.client.profile_statuses_article(uid, page).await
    }
}

impl WeiboAPI for MockAPI {}

#[cfg(test)]
mod local_tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn get_test_data_path(file_name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/")
            .join(file_name)
    }

    #[tokio::test]
    async fn test_login_flow() {
        let mock_client = MockClient::new();
        let mut api = MockAPI::new(mock_client.clone());

        // get_sms_code
        mock_client
            .set_get_sms_code_response_from_file(&get_test_data_path("get_sms_code.json"))
            .unwrap();
        api.get_sms_code("12345678901".to_owned()).await.unwrap();

        // login
        mock_client
            .set_login_response_from_file(&get_test_data_path("login.json"))
            .unwrap();
        api.login("123456").await.unwrap();
        assert!(matches!(api.login_state(), &LoginState::LoggedIn { .. }));
    }

    #[tokio::test]
    async fn test_login_with_session() {
        let mock_client = MockClient::new();
        let mut api = MockAPI::new(mock_client.clone());
        let session = Session {
            gsid: "gsid".to_string(),
            uid: "uid".to_string(),
            screen_name: "screen_name".to_string(),
            cookie_store: Default::default(),
        };
        let session = Arc::new(Mutex::new(session));

        mock_client
            .set_login_response_from_file(&get_test_data_path("login.json"))
            .unwrap();
        api.login_with_session(session).await.unwrap();
        assert!(matches!(api.login_state(), &LoginState::LoggedIn { .. }));
    }

    fn create_logged_in_api() -> (MockClient, MockAPI) {
        let mock_client = MockClient::new();
        let session = Session {
            gsid: "gsid".to_string(),
            uid: "uid".to_string(),
            screen_name: "screen_name".to_string(),
            cookie_store: Default::default(),
        };
        let session = Arc::new(Mutex::new(session));
        let api = MockAPI::from_session(mock_client.clone(), session);
        (mock_client, api)
    }

    #[tokio::test]
    async fn test_emoji_update() {
        let (mock_client, api) = create_logged_in_api();
        mock_client
            .set_emoji_update_response_from_file(&get_test_data_path("emoji.json"))
            .unwrap();
        mock_client
            .set_web_emoticon_response_from_file(&get_test_data_path("web_emoji.json"))
            .unwrap();
        let result = api.emoji_update().await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_favorites() {
        let (mock_client, api) = create_logged_in_api();
        mock_client
            .set_favorites_response_from_file(&get_test_data_path("favorites.json"))
            .unwrap();
        let result = api.favorites(1).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_favorites_destroy() {
        let (mock_client, api) = create_logged_in_api();
        mock_client.set_favorites_destroy_response_from_str("");
        api.favorites_destroy(123).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_statuses_show() {
        let (mock_client, api) = create_logged_in_api();
        mock_client
            .set_statuses_show_response_from_file(&get_test_data_path("statuses_show.json"))
            .unwrap();
        let result = api.statuses_show(123).await.unwrap();
        assert!(!result.long_text.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_profile_statuses() {
        let (mock_client, api) = create_logged_in_api();
        mock_client
            .set_profile_statuses_response_from_file(&get_test_data_path("profile_statuses.json"))
            .unwrap();
        let result = api.profile_statuses(1786055427, 1).await.unwrap();
        assert!(!result.is_empty());
    }
}
