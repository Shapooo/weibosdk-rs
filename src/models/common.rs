use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PicInfoDetail {
    #[serde(deserialize_with = "deserialize_str_num")]
    pub height: i32,
    #[serde(deserialize_with = "deserialize_str_num")]
    pub width: i32,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HugeInfo {
    pub author_id: String,
    pub content1: String,
    pub content2: String,
    pub media_info: VideoInfo,
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VideoInfo {
    pub author_mid: Option<String>,
    pub author_name: Option<String>,
    pub big_pic_info: Option<PicInfoItemSimple>,
    pub duration: i32,
    pub format: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub h265_mp4_hd: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub h265_mp4_ld: Option<String>,
    pub h5_url: String,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub hevc_mp4_720p: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub inch_4_mp4_hd: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub inch_5_5_mp4_hd: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub inch_5_mp4_hd: Option<String>,
    pub is_short_video: Option<i32>,
    pub kol_title: Option<String>,
    pub media_id: String,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub mp4_720p_mp4: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub mp4_hd_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub mp4_sd_url: Option<String>,
    pub name: Option<String>,
    pub next_title: Option<String>,
    pub online_users: Option<String>,
    pub online_users_number: Option<i32>,
    pub origin_total_bitrate: Option<i32>,
    pub prefetch_size: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub stream_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nonable_str")]
    pub stream_url_hd: Option<String>,
    pub video_orientation: Option<Orientation>,
    pub video_publish_time: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PicInfoItemSimple {
    pub pic_big: PicInfoDetail,
    pub pic_middle: PicInfoDetail,
    pub pic_small: PicInfoDetail,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

pub fn deserialize_nonable_str<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok((!s.is_empty()).then_some(s))
}
