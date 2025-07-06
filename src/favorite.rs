#![allow(async_fn_in_trait)]
use crate::weibo_api::WeiboAPI;

pub trait FavoriteAPI {
    async fn favorites() -> ();
    async fn favorites_create() -> ();
}

impl FavoriteAPI for WeiboAPI {
    async fn favorites() -> () {
        todo!()
    }

    async fn favorites_create() -> () {
        todo!()
    }
}
