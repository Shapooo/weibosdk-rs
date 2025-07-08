use std::fmt;

#[derive(Debug)]
pub enum LoginError {
    SessionNotFound,
    SessionInvalid,
    NetworkError(anyhow::Error),
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginError::SessionNotFound => write!(f, "Session not found"),
            LoginError::SessionInvalid => write!(f, "Session invalid"),
            LoginError::NetworkError(err) => write!(f, "Network error: {}", err),
        }
    }
}

impl std::error::Error for LoginError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoginError::NetworkError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<anyhow::Error> for LoginError {
    fn from(err: anyhow::Error) -> Self {
        LoginError::NetworkError(err)
    }
}
