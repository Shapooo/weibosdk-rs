use thiserror::Error;

use crate::models::err_response::ErrResponse;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Network request failed: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("API returned an error: {0:?}")]
    ApiError(ErrResponse),

    #[error("Failed to deserialize response: {0}")]
    DeserializationError(#[from] serde_json::Error),

    #[error("Failed to convert data: {0}")]
    DataConversionError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unlogged in")]
    NotLoggedIn,
}

pub type Result<T> = std::result::Result<T, Error>;
