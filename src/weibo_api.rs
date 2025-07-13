use crate::{client::HttpClient, session::Session};

#[derive(Debug)]
pub struct WeiboAPIImpl<C: HttpClient> {
    pub client: C,
    session: Session,
}

impl<C: HttpClient> WeiboAPIImpl<C> {
    pub fn new(client: C, session: Session) -> Self {
        WeiboAPIImpl { client, session }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }
}

impl<C: HttpClient> crate::WeiboAPI for WeiboAPIImpl<C> {}
