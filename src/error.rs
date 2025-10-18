use rmcp::ErrorData;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid input: {0}")]
    Invalid(String),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    Keyring(#[from] keyring::Error),
    #[error(transparent)]
    Mcp(#[from] ErrorData),
}

impl From<Error> for ErrorData {
    fn from(err: Error) -> Self {
        match err {
            Error::Invalid(msg) => ErrorData::invalid_params(msg, None),
            other => ErrorData::internal_error(other.to_string(), None),
        }
    }
}

impl Error {
    pub fn invalid<S: Into<String>>(msg: S) -> Self {
        Self::Invalid(msg.into())
    }
}
