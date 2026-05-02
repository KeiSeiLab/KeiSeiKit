//! Shared HTTP utilities: a process-wide `reqwest::Client` and a capped
//! response-body reader.
//!
//! A single `reqwest::Client` is reused for all outbound calls to avoid
//! exhausting OS connection-table entries when the daemon is under load.
//! The client is initialized once via `once_cell::sync::Lazy`.

use once_cell::sync::Lazy;

/// The process-wide HTTP client.  Shared by `anthropic`, `anthropic_invoker`,
/// `elevenlabs`, and `fal` so that connection pooling is effective.
pub static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .build()
        .expect("reqwest::Client::builder().build() — no TLS config error expected")
});

/// Error returned when a response body exceeds the caller-supplied cap.
#[derive(Debug)]
pub struct BodyTooLarge;

impl std::fmt::Display for BodyTooLarge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "response body exceeded size cap")
    }
}

impl std::error::Error for BodyTooLarge {}

/// Read a response body up to `max_bytes`. Returns `Err(BodyTooLarge)` if
/// the stream exceeds the cap; the partial buffer is discarded on overflow
/// so the caller does not accidentally use truncated data.
pub async fn read_capped(
    resp: reqwest::Response,
    max_bytes: usize,
) -> Result<Vec<u8>, CappedReadError> {
    use futures::StreamExt;
    let mut bytes: Vec<u8> = Vec::new();
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(CappedReadError::Http)?;
        if bytes.len() + chunk.len() > max_bytes {
            return Err(CappedReadError::TooLarge);
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

/// Combined error for `read_capped`.
#[derive(Debug, thiserror::Error)]
pub enum CappedReadError {
    #[error("response body exceeded size cap")]
    TooLarge,
    #[error("http stream error: {0}")]
    Http(reqwest::Error),
}
