use serde::{Deserialize, Serialize};

use super::PicInfoDetail;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PicInfoItem {
    pub bmiddle: PicInfoDetail,
    pub large: PicInfoDetail,
    pub focus_point: Option<FocusPoint>,
    pub largest: PicInfoDetail,
    pub mw2000: PicInfoDetail,
    pub original: PicInfoDetail,
    pub object_id: String,
    pub photo_tag: i32,
    pub pic_id: String,
    pub pic_status: i32,
    pub r#type: PicInfoType,
    pub thumbnail: PicInfoDetail,
    pub video: Option<String>,
    pub video_object_id: Option<String>,
    pub fid: Option<String>,
    pub video_hd: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FocusPoint {
    pub height: f32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PicInfoType {
    #[serde(rename = "pic")]
    Pic,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "livephoto")]
    Livephoto,
}
