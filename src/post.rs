use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use serde_json::Value;

use crate::User;
use crate::utils::serialize_datetime;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Post {
    pub id: i64,
    pub mblogid: String,
    pub source: Option<String>,
    pub region_name: Option<String>,
    pub deleted: bool,
    pub pic_ids: Option<Vec<String>>,
    pub pic_num: Option<i64>,
    pub url_struct: Option<Value>,
    pub topic_struct: Option<Value>,
    pub tag_struct: Option<Value>,
    pub number_display_strategy: Option<Value>,
    pub mix_media_info: Option<Value>,
    pub text: String,
    pub attitudes_status: i64,
    pub favorited: bool,
    pub pic_infos: Option<HashMap<String, Value>>,
    pub reposts_count: Option<i64>,
    pub comments_count: Option<i64>,
    pub attitudes_count: Option<i64>,
    pub repost_type: Option<i64>,
    pub edit_count: Option<i64>,
    pub is_long_text: bool,
    pub geo: Option<Value>,
    pub page_info: Option<Value>,
    pub unfavorited: bool,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<FixedOffset>,
    pub retweeted_status: Option<Box<Post>>,
    pub user: Option<User>,
}
