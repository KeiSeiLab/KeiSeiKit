use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("signature error: {0}")]
    Signature(String),

    #[error("hex decode error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("invalid key length: expected 32 bytes, got {0}")]
    KeyLength(usize),

    #[error("field contains forbidden separator '|': {0}")]
    MessageSeparator(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<ed25519_dalek::ed25519::Error> for Error {
    fn from(e: ed25519_dalek::ed25519::Error) -> Self {
        Error::Signature(e.to_string())
    }
}
