#![allow(async_fn_in_trait)]
pub trait FavoriteAPI {
    async fn favorites() -> ();
    async fn favorites_create() -> ();
}

impl FavoriteAPI for super::client::WeiboClient {
    async fn favorites() -> () {
        todo!()
    }

    async fn favorites_create() -> () {
        todo!()
    }
}
