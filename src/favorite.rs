#![allow(async_fn_in_trait)]
use crate::client::HttpClient;
use crate::weibo_api::WeiboAPI;

pub trait FavoriteAPI<C: HttpClient> {
    async fn favorites() -> ();
    async fn favorites_create() -> ();
}

impl<C: HttpClient> FavoriteAPI<C> for WeiboAPI<C> {
    async fn favorites() -> () {
        todo!()
    }

    async fn favorites_create() -> () {
        todo!()
    }
}
