use serde::{Deserialize, Serialize};

use super::HugeInfo;
use super::PicInfoItem;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MixMediaInfo {
    pub items: Vec<MixMediaInfoItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MixMediaInfoItem {
    #[serde(rename = "pic")]
    Pic { id: String, data: Box<PicInfoItem> },
    #[serde(rename = "video")]
    Video { id: String, data: Box<HugeInfo> },
}
