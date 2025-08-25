use std::borrow::Cow;
use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{mix_media_info::MixMediaInfo, page_info::PageInfo, url_struct::UrlStruct};
use crate::User;
use crate::models::pic_infos::PicInfoItem;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Post {
    pub attitudes_count: Option<i64>,
    #[serde(default)]
    pub attitudes_status: i64,
    #[serde(with = "datetime")]
    pub created_at: DateTime<FixedOffset>,
    pub comments_count: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_deleted")]
    pub deleted: bool,
    pub edit_count: Option<i64>,
    #[serde(default)]
    pub favorited: bool,
    pub geo: Option<Value>,
    pub id: i64,
    #[serde(default, rename = "isLongText")]
    pub is_long_text: bool,
    #[serde(
        default,
        rename = "longText",
        deserialize_with = "deserialize_long_text"
    )]
    pub long_text: Option<String>,
    pub mblogid: String,
    #[serde(default, deserialize_with = "deserialize_ids")]
    pub mix_media_ids: Option<Vec<String>>,
    pub mix_media_info: Option<MixMediaInfo>,
    pub page_info: Option<PageInfo>,
    #[serde(default, deserialize_with = "deserialize_ids")]
    pub pic_ids: Option<Vec<String>>,
    pub pic_infos: Option<HashMap<String, PicInfoItem>>,
    pub pic_num: Option<i64>,
    pub region_name: Option<String>,
    pub reposts_count: Option<i64>,
    pub repost_type: Option<i64>,
    pub retweeted_status: Option<Box<Post>>,
    pub source: Option<String>,
    pub text: String,
    #[serde(default)]
    pub unfavorited: bool,
    pub url_struct: Option<UrlStruct>,
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
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrBool<'a> {
        B(bool),
        S(Cow<'a, str>),
    }

    let res = StrBool::deserialize(deserializer)?;
    match res {
        StrBool::S(c) => Ok(c == "1"),
        StrBool::B(b) => Ok(b),
    }
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

pub fn deserialize_long_text<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct LongText {
        pub content: String,
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum LTorS<'a> {
        LT(LongText),
        S(Cow<'a, str>),
    }

    let res = Option::<LTorS>::deserialize(deserializer)?.map(|lts| match lts {
        LTorS::S(s) => s.to_string(),
        LTorS::LT(lt) => lt.content,
    });
    Ok(res)
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
        DateTime::parse_from_str(&created_at, "%a %b %d %T %z %Y")
            .or_else(|_| DateTime::parse_from_rfc3339(&created_at))
            .map_err(serde::de::Error::custom)
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
