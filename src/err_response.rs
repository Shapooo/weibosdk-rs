use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ErrResponse {
    pub errmsg: String,
    pub errno: i32,
    pub errtype: String,
    pub isblock: bool,
}
