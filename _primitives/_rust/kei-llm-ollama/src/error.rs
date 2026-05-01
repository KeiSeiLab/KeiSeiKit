//! Error type for the Ollama HTTP adapter.
//!
//! Maps to deterministic exit codes:
//! - 0 success
//! - 1 IO/decode/transport error
//! - 2 daemon-not-running OR model-not-found
//! - 3 timeout

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    /// Connection refused on localhost:11434 — Ollama daemon is not up.
    #[error("Ollama daemon not running at {url}: {source}\n  hint: run `ollama serve` or `brew services start ollama`")]
    DaemonNotRunning {
        url: String,
        #[source]
        source: std::io::Error,
    },

    /// 404 from /api/generate or /api/chat — model not pulled.
    #[error("model not found: {0} (try `ollama pull {0}` or `kei-llm-ollama pull --model {0}`)")]
    ModelNotFound(String),

    /// Non-2xx status that is not a 404.
    #[error("HTTP {status} from Ollama: {body}")]
    HttpError { status: u16, body: String },

    /// JSON decode failure on response body.
    #[error("decode error: {0}")]
    DecodeError(String),

    /// Network or library-level transport error (not a connection refused).
    #[error("transport error: {0}")]
    Transport(String),

    /// Request exceeded timeout budget.
    #[error("request timed out after {ms} ms")]
    Timeout { ms: u64 },
}

impl ApiError {
    /// Map error to process exit code.
    pub fn exit_code(&self) -> i32 {
        match self {
            ApiError::DaemonNotRunning { .. } | ApiError::ModelNotFound(_) => 2,
            ApiError::Timeout { .. } => 3,
            _ => 1,
        }
    }
}

/// Classify a reqwest error: connection-refused → DaemonNotRunning, timeout → Timeout, else Transport.
pub fn classify_reqwest_error(err: reqwest::Error, url: &str, timeout_ms: u64) -> ApiError {
    if err.is_timeout() {
        return ApiError::Timeout { ms: timeout_ms };
    }
    if err.is_connect() {
        let io = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, err.to_string());
        return ApiError::DaemonNotRunning {
            url: url.into(),
            source: io,
        };
    }
    ApiError::Transport(err.to_string())
}
