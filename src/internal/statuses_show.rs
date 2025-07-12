use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LongText {
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditConfig {
    #[allow(unused)]
    pub edited: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatusesShow {
    #[allow(unused)]
    pub edit_config: EditConfig,
    #[serde(rename = "longText")]
    pub long_text: LongText,
}
