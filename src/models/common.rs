use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PicInfoDetail {
    #[serde(deserialize_with = "deserialize_str_num")]
    pub height: i32,
    #[serde(deserialize_with = "deserialize_str_num")]
    pub width: i32,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HugeInfo {
    pub author_id: String,
    pub content1: String,
    pub content2: String,
    pub media_info: MediaInfo,
    pub object_id: String,
    pub object_type: String,
    pub oid: String,
    pub page_id: String,
    pub page_pic: String,
    pub page_title: String,
    pub page_url: String,
    pub pic_info: PicInfoItemSimple,
    pub short_url: String,
    pub r#type: String,
    pub type_icon: String,
    pub warn: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaInfo {
    pub author_mid: String,
    pub author_name: String,
    pub belong_collection: i32,
    pub big_pic_info: PicInfoItemSimple,
    pub duration: i32,
    pub format: String,
    pub h265_mp4_hd: String,
    pub h265_mp4_ld: String,
    pub h5_url: String,
    pub hevc_mp4_720p: String,
    pub inch_4_mp4_hd: String,
    pub inch_5_5_mp4_hd: String,
    pub inch_5_mp4_hd: String,
    pub is_short_video: i32,
    pub jump_to: i32,
    pub kol_title: String,
    pub media_id: String,
    pub mp4_720p_mp4: String,
    pub mp4_hd_url: String,
    pub mp4_sd_url: String,
    pub name: String,
    pub next_title: String,
    pub online_users: String,
    pub online_users_number: i32,
    pub origin_total_bitrate: i32,
    pub play_loop_type: i32,
    pub prefetch_size: i32,
    pub prefetch_type: i32,
    pub protocol: String,
    pub search_scheme: String,
    pub stream_url: String,
    pub stream_url_hd: String,
    #[serde(deserialize_with = "deserialize_str_num")]
    pub titles_display_time: i32,
    pub ttl: i32,
    pub video_orientation: Orientation,
    pub video_publish_time: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PicInfoItemSimple {
    pub pic_big: PicInfoDetail,
    pub pic_middle: PicInfoDetail,
    pub pic_small: PicInfoDetail,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Orientation {
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "horizontal")]
    Horizontal,
}

pub fn deserialize_str_num<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Either<'a> {
        Str(Cow<'a, str>),
        Num(i32),
    }
    match Either::deserialize(deserializer)? {
        Either::Str(s) => s.parse().map_err(serde::de::Error::custom),
        Either::Num(n) => Ok(n),
    }
}
