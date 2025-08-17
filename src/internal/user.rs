use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::User;
use crate::error::{Error, Result};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserInternal {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub screen_name: String,
    #[serde(default)]
    pub profile_image_url: String,
    #[serde(default)]
    pub avatar_large: String,
    #[serde(default)]
    pub avatar_hd: String,
    #[serde(default)]
    pub pc_new: i64,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub verified_type: i64,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub weihao: String,
    pub verified_type_ext: Option<i64>,
    #[serde(default)]
    pub follow_me: bool,
    #[serde(default)]
    pub following: bool,
    #[serde(default)]
    pub mbrank: i64,
    #[serde(default)]
    pub mbtype: i64,
}

impl TryFrom<Value> for UserInternal {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        Ok(serde_json::from_value(value)?)
    }
}

impl From<UserInternal> for User {
    fn from(value: UserInternal) -> Self {
        User {
            id: value.id,
            screen_name: value.screen_name,
            profile_image_url: value.profile_image_url,
            avatar_large: value.avatar_large,
            avatar_hd: value.avatar_hd,
            verified: value.verified,
            verified_type: value.verified_type,
            domain: value.domain,
            follow_me: value.follow_me,
            following: value.following,
        }
    }
}

impl TryInto<Value> for UserInternal {
    type Error = serde_json::Error;

    fn try_into(self) -> std::result::Result<Value, Self::Error> {
        serde_json::to_value(self)
    }
}

#[cfg(test)]
mod local_tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_deserialize_user_internal() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let testcase_path = manifest_dir.join("tests/data/favorites.json");
        let response_body = std::fs::read_to_string(testcase_path).unwrap();
        let mut value = serde_json::from_str::<Value>(&response_body).unwrap();
        if let Value::Array(v) = value["favorites"].take() {
            value = v
                .into_iter()
                .map(|mut post| post["status"]["user"].take())
                .filter(|user| !user.is_null() && !user["id"].is_null())
                .collect();
        }
        let _ = serde_json::from_value::<Vec<UserInternal>>(value).unwrap();
    }
}
