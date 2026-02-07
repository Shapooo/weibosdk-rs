use log::info;

use crate::{
    api_client::ApiClient,
    constants::{params::*, urls::*},
    error::Result,
    http_client::HttpClient,
    utils,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ContainerType {
    #[default]
    Normal,
    Original,
    Picture,
    Video,
    Article,
}

impl ContainerType {
    pub fn to_container_id(&self, uid: i64) -> String {
        match self {
            Self::Normal => format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO"),
            Self::Original => format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO_ORI"),
            Self::Picture => format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO_PIC"),
            Self::Video => format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO_VIDEO"),
            Self::Article => format!("230413{uid}_-_WEIBO_SECOND_PROFILE_WEIBO_ARTICAL"),
        }
    }
}

impl<C: HttpClient> ApiClient<C> {
    pub async fn profile_statuses(
        &self,
        uid: i64,
        page: u32,
        container_type: ContainerType,
    ) -> Result<C::Response> {
        info!(
            "getting profile statuses, uid: {uid}, page: {page}, containerid: {container_type:?}"
        );
        let session = self.session()?;
        let s = utils::generate_s(&session.uid, FROM);
        let mut params = utils::build_common_params();
        params["gsid"] = session.gsid.clone().into();
        params["s"] = s.into();
        params["uid"] = uid.into();
        params["page"] = page.into();
        params["count"] = self.config.status_count.into();
        params["mix_media_enable"] = MIX_MEDIA_ENABLE.into();
        params["containerid"] = container_type.to_container_id(uid).into();
        self.client
            .get(URL_PROFILE_STATUSES, &params, self.config.retry_times)
            .await
    }
}

#[cfg(test)]
mod real_tests {
    use super::*;
    use crate::{api_client::ApiClient, http_client, session::Session};

    #[tokio::test]
    async fn test_real_profile_statuses() {
        let session_file = "session.json";
        if let Ok(session) = Session::load(session_file) {
            let client = http_client::Client::new().unwrap();
            let weibo_api = ApiClient::from_session(client, session);
            let _posts = weibo_api
                .profile_statuses(1401527553, 1, ContainerType::Normal)
                .await
                .unwrap();
        }
    }
}
