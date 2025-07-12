#![allow(unused)]
macro_rules! full_url {
    ($path:expr) => {
        concat!("https://api.weibo.cn", $path)
    };
}

pub mod urls {
    pub const URL_FAVORITES: &str = full_url!("/2/favorites");
    pub const URL_SEND_CODE: &str = full_url!("/2/account/login_sendcode");
    pub const URL_LOGIN: &str = full_url!("/2/account/login");
    pub const URL_FAVORITES_DESTROY: &str = full_url!("/2/favorites/destroy");
    pub const URL_BUILD_COMMENTS: &str = full_url!("/2/comments/build_comments");
    pub const URL_STATUSES_SHOW: &str = full_url!("2/2statuses/show");
    pub const URL_PROFILE_STATUSES: &str = full_url!("/2/profile/statuses");
}

// Common Parameters
pub mod params {
    pub const FROM: &str = "12DC195010";
    pub const SESSION_REFRESH_FROM: &str = "1299295010";
    pub const LANG: &str = "zh_CN";
    pub const LOCALE: &str = "zh_CN";
    pub const UA: &str = "HONOR-PGT-AN10_9_WeiboIntlAndroid_6710";
    pub const WM: &str = "2468_1001";
    pub const PARAM_C: &str = "weicoabroad";
    pub const COUNT: u8 = 20;
    pub const SOURCE: &str = "4215535043";
    pub const MIX_MEDIA_ENABLE: u8 = 1;
}

pub mod config {
    pub const RETRY_TIMES: u8 = 3;
}
