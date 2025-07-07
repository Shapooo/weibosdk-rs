#[derive(Debug)]
pub enum LoginError {
    SessionNotFound,
    SessionInvalid,
    NetworkError(anyhow::Error),
}

impl From<anyhow::Error> for LoginError {
    fn from(err: anyhow::Error) -> Self {
        LoginError::NetworkError(err)
    }
    // ... other errors
}
