use log::{debug, info};

use crate::{
    api_client::ApiClient,
    constants::{
        params::*,
        urls::{URL_FAVORITES, URL_FAVORITES_DESTROY},
    },
    error::Result,
    http_client::HttpClient,
    utils,
};

impl<C: HttpClient> ApiClient<C> {
    pub async fn favorites(&self, page: u32) -> Result<C::Response> {
        info!("getting favorites, page: {page}");
        let session = self.session()?;
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["page"] = page.into();
        params["count"] = self.config.fav_count.into();
        params["mix_media_enable"] = MIX_MEDIA_ENABLE.into();

        self.client
            .get(URL_FAVORITES, &params, self.config.retry_times)
            .await
    }

    pub async fn favorites_destroy(&self, id: i64) -> Result<()> {
        info!("destroying favorite, id: {id}");
        let session = self.session()?;
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["id"] = id.into();
        let _ = self
            .client
            .post(URL_FAVORITES_DESTROY, &params, self.config.retry_times)
            .await?;
        debug!("favorite {id} destroyed");
        Ok(())
    }
}

#[cfg(test)]
mod real_tests {
    use crate::{api_client::ApiClient, http_client, session::Session};

    #[tokio::test]
    async fn test_real_favorites() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let client = http_client::Client::new().unwrap();
            let weibo_api = ApiClient::from_session(client, session);
            let _ = weibo_api.favorites(1).await.unwrap();
        }
    }
}
