use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ErrResponse {
    pub errmsg: String,
    pub errno: i32,
    pub errtype: String,
    pub isblock: bool,
}
