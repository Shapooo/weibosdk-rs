use log::info;

use crate::{
    api_client::ApiClient,
    constants::{params::*, urls::URL_STATUSES_SHOW},
    error::Result,
    http_client::HttpClient,
    utils,
};

impl<C: HttpClient> ApiClient<C> {
    pub async fn statuses_show(&self, id: i64) -> Result<C::Response> {
        info!("getting long text, id: {id}");
        let session = self.session()?;
        let session = session.lock().unwrap().clone();
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["id"] = id.into();
        params["isGetLongText"] = 1.into();

        self.client
            .get(URL_STATUSES_SHOW, &params, self.config.retry_times)
            .await
    }
}

#[cfg(test)]
mod real_tests {
    use crate::{api_client::ApiClient, http_client, session::Session};
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_real_get_statuses_show() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let session = Arc::new(Mutex::new(session));
            let client = http_client::Client::new().unwrap();
            let weibo_api = ApiClient::from_session(client, session);
            let _post = weibo_api.statuses_show(5179586393932632).await.unwrap();
        }
    }
}
