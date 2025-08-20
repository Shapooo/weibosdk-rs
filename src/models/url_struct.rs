use std::collections::HashMap;
use std::result::Result;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::PicInfoDetail;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlStruct(pub Vec<UrlStructItem>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlStructItem {
    pub hide: Option<u8>,
    pub long_url: Option<String>,
    pub object_type: Option<String>,
    pub ori_url: String,
    pub page_id: Option<String>,
    pub result: bool,
    pub short_url: String,
    pub url_title: String,
    #[serde(default, deserialize_with = "deserialize_url_type")]
    pub url_type: Option<u8>,
    pub url_type_pic: Option<String>,
    #[serde(default, deserialize_with = "deserialize_pic_ids")]
    pub pic_ids: Option<String>,
    #[serde(default, deserialize_with = "deserialize_pic_infos")]
    pub pic_infos: Option<PicInfosForStatus>,
    pub vip_gif: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PicInfosForStatus {
    pub bmiddle: PicInfoDetail,
    pub large: PicInfoDetail,
    pub thumbnail: PicInfoDetail,
    pub woriginal: PicInfoDetail,
}

fn deserialize_url_type<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(u8::deserialize(deserializer).ok())
}

fn deserialize_pic_ids<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Vec::<String>::deserialize(deserializer)?.into_iter().next())
}

fn deserialize_pic_infos<'de, D>(deserializer: D) -> Result<Option<PicInfosForStatus>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(
        HashMap::<String, PicInfosForStatus>::deserialize(deserializer)?
            .into_values()
            .next(),
    )
}
