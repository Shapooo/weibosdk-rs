use serde::{Deserialize, Serialize};

use super::HugeInfo;
use super::PicInfoItem;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MixMediaInfo {
    pub items: Vec<MixMediaInfoItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MixMediaInfoItem {
    pub data: MixMediaInfoData,
    pub id: String,
    pub scheme: String,
    pub r#type: MixMediaInfoType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MixMediaInfoData {
    PicInfo(Box<PicInfoItem>),
    HugeInfo(Box<HugeInfo>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MixMediaInfoType {
    #[serde(rename = "pic")]
    Pic,
    #[serde(rename = "video")]
    Video,
}
