use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::User;
use crate::models::pic_infos::PicInfoItem;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    pub id: i64,
    pub mblogid: String,
    pub source: Option<String>,
    pub region_name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_deleted")]
    pub deleted: bool,
    #[serde(default, deserialize_with = "deserialize_ids")]
    pub pic_ids: Option<Vec<String>>,
    pub pic_num: Option<i64>,
    pub url_struct: Option<Value>,
    pub topic_struct: Option<Value>,
    pub tag_struct: Option<Value>,
    pub common_struct: Option<Value>,
    #[serde(default, deserialize_with = "deserialize_ids")]
    pub mix_media_ids: Option<Vec<String>>,
    pub mix_media_info: Option<Value>,
    pub text: String,
    #[serde(default)]
    pub attitudes_status: i64,
    #[serde(default)]
    pub favorited: bool,
    pub pic_infos: Option<HashMap<String, PicInfoItem>>,
    pub reposts_count: Option<i64>,
    pub comments_count: Option<i64>,
    pub attitudes_count: Option<i64>,
    pub repost_type: Option<i64>,
    pub edit_count: Option<i64>,
    #[serde(default, rename = "isLongText")]
    pub is_long_text: bool,
    pub geo: Option<Value>,
    pub page_info: Option<Value>,
    #[serde(default)]
    pub unfavorited: bool,
    #[serde(with = "datetime")]
    pub created_at: DateTime<FixedOffset>,
    pub retweeted_status: Option<Box<Post>>,
    #[serde(default, deserialize_with = "deserialize_user")]
    pub user: Option<User>,
}

fn deserialize_user<'de, D>(deserializer: D) -> std::result::Result<Option<User>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let user = Option::<User>::deserialize(deserializer)?;
    Ok(user.and_then(|u| if u.id == 0 { None } else { Some(u) }))
}

fn deserialize_deleted<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    Ok(str == "1")
}

pub fn deserialize_ids<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let ids = Option::<Vec<String>>::deserialize(deserializer)?;
    Ok(ids.and_then(|ids| if ids.is_empty() { None } else { Some(ids) }))
}

mod datetime {
    use std::borrow::Cow;

    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(dt: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let created_at = Cow::<'_, str>::deserialize(deserializer)?;
        match DateTime::parse_from_str(&created_at, "%a %b %d %T %z %Y") {
            Ok(dt) => Ok(dt),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}

#[cfg(test)]
mod local_tests {
    use serde_json::from_value;

    use super::*;
    use std::path::Path;

    #[test]
    fn test_deserialize_post() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let testcase_path = manifest_dir.join("tests/data/favorites.json");
        let response_body = std::fs::read_to_string(testcase_path).unwrap();
        let mut value = serde_json::from_str::<Value>(&response_body).unwrap();
        value = value["favorites"].take();
        if let Value::Array(v) = value.take() {
            let _ = v
                .into_iter()
                .map(|mut post| from_value::<Post>(post["status"].take()))
                .collect::<Vec<_>>();
        }
    }
}
