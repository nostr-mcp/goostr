use rmcp::ErrorData;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoostrError {
    #[error("invalid input: {0}")]
    Invalid(String),

    #[error("no active key set")]
    NoActiveKey,

    #[error("key not found: {0}")]
    KeyNotFound(String),

    #[error("invalid event ID: {0}")]
    InvalidEventId(String),

    #[error("invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("invalid kind number: {0}")]
    InvalidKind(u16),

    #[error("missing required parameter: {0}")]
    MissingParameter(String),

    #[error("relay error: {0}")]
    Relay(String),

    #[error("event publishing failed: {0}")]
    PublishFailed(String),

    #[error("nostr protocol error: {0}")]
    NostrProtocol(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),

    #[error(transparent)]
    Mcp(#[from] ErrorData),
}

impl From<GoostrError> for ErrorData {
    fn from(err: GoostrError) -> Self {
        match err {
            GoostrError::Invalid(msg) => ErrorData::invalid_params(msg, None),
            GoostrError::NoActiveKey => ErrorData::invalid_params("no active key set", None),
            GoostrError::KeyNotFound(label) => {
                ErrorData::invalid_params(format!("key not found: {}", label), None)
            }
            GoostrError::InvalidEventId(id) => {
                ErrorData::invalid_params(format!("invalid event ID: {}", id), None)
            }
            GoostrError::InvalidPublicKey(pk) => {
                ErrorData::invalid_params(format!("invalid public key: {}", pk), None)
            }
            GoostrError::InvalidKind(kind) => {
                ErrorData::invalid_params(format!("invalid kind number: {}", kind), None)
            }
            GoostrError::MissingParameter(param) => {
                ErrorData::invalid_params(format!("missing required parameter: {}", param), None)
            }
            GoostrError::Relay(msg) => ErrorData::internal_error(format!("relay error: {}", msg), None),
            GoostrError::PublishFailed(msg) => {
                ErrorData::internal_error(format!("publishing failed: {}", msg), None)
            }
            GoostrError::NostrProtocol(msg) => {
                ErrorData::internal_error(format!("nostr protocol error: {}", msg), None)
            }
            other => ErrorData::internal_error(other.to_string(), None),
        }
    }
}

impl GoostrError {
    pub fn invalid<S: Into<String>>(msg: S) -> Self {
        Self::Invalid(msg.into())
    }

    pub fn missing_param<S: Into<String>>(param: S) -> Self {
        Self::MissingParameter(param.into())
    }
}
