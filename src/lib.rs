pub mod client;
pub mod config;
pub mod emoji;
pub mod err_response;
pub mod error;
pub mod favorites;
pub mod long_text;
pub mod post;
pub mod profile_statuses;
pub mod session;
pub mod user;
pub mod weibo_api;

mod constants;
mod internal;
mod utils;

#[cfg(test)]
mod mock_client;

pub use error::{Error, Result};
pub use post::Post;
pub use user::User;
pub use weibo_api::WeiboAPIImpl;

pub trait WeiboAPI:
    emoji::EmojiUpdateAPI
    + favorites::FavoritesAPI
    + long_text::LongTextAPI
    + profile_statuses::ProfileStatusesAPI
    + Send
    + Sync
    + Clone
{
}
