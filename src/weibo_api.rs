use anyhow::Result;
use log::info;
use std::path::{Path, PathBuf};

use crate::{
    client::HttpClient,
    login::{Login, SendCode},
    session::Session,
};

//-------------------------------------------------------------
//---------------------- WeiboClient --------------------------
//-------------------------------------------------------------

#[derive(Debug)]
pub struct WeiboAPI<C: HttpClient> {
    client: C,
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
