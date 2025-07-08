use crate::{client::HttpClient, session::Session};

//-------------------------------------------------------------
//---------------------- WeiboClient --------------------------
//-------------------------------------------------------------

#[derive(Debug)]
pub struct WeiboAPI<C: HttpClient> {
    pub client: C,
    session: Session,
}

impl<C: HttpClient> WeiboAPI<C> {
    pub fn new(client: C, session: Session) -> Self {
        WeiboAPI { client, session }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }
}
