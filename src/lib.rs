pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod session;

mod constants;
mod cookie;
mod emoji;
mod favorites;
mod profile_statuses;
mod utils;
mod weibo_api;

#[cfg(any(feature = "test-mocks", test))]
pub mod mock;

pub use client::Client;
pub use emoji::EmojiUpdateAPI;
pub use error::{Error, Result};
pub use favorites::FavoritesAPI;
pub use models::long_text::LongTextAPI;
pub use models::{post::Post, user::User};
pub use profile_statuses::ProfileStatusesAPI;
pub use weibo_api::WeiboAPIImpl;

pub trait WeiboAPI:
    emoji::EmojiUpdateAPI
    + favorites::FavoritesAPI
    + models::long_text::LongTextAPI
    + profile_statuses::ProfileStatusesAPI
    + Send
    + Sync
    + Clone
{
}
