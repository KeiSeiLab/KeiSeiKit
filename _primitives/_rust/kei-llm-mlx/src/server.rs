//! Local OpenAI-compat HTTP server — `mlx_lm.server`.
//!
//! Constructor Pattern: this cube builds the spawn argv and returns a
//! `ServerHandle` describing the still-attached child PID + bound URL.
//! Bind is FORCED to localhost — `--host 0.0.0.0` (or any non-loopback
//! literal) is rejected with `Error::SecurityRefused`. Remote-binding
//! decisions belong to the operator with explicit configuration, not to
//! this primitive.

use crate::error::Error;
use crate::platform::is_supported;
use serde::{Deserialize, Serialize};

pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 8080;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSpec {
    pub model_id: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHandle {
    pub pid: u32,
    pub host: String,
    pub port: u16,
    pub openai_compat_url: String,
    pub argv: Vec<String>,
}

/// Validate spec + build argv. Does NOT spawn — callers spawn through
/// `Runner` (or a thin std::process call in main) so tests stay
/// deterministic.
pub fn build_spec(model_id: &str, host: &str, port: u16) -> Result<ServerSpec, Error> {
    let support = is_supported();
    if !support.supported {
        return Err(Error::NotSupported(
            support.reason.unwrap_or_else(|| "unsupported".into()),
        ));
    }
    if !is_localhost(host) {
        return Err(Error::SecurityRefused(format!(
            "host `{host}` is not localhost; refusing remote bind"
        )));
    }
    Ok(ServerSpec { model_id: model_id.to_string(), host: host.to_string(), port })
}

/// Build argv for `mlx_lm.server`. Visible for tests.
pub fn build_argv(spec: &ServerSpec) -> Vec<String> {
    vec![
        "--model".into(),
        spec.model_id.clone(),
        "--host".into(),
        spec.host.clone(),
        "--port".into(),
        spec.port.to_string(),
    ]
}

/// Compose the OpenAI-compat URL the consumer will hit.
pub fn openai_compat_url(spec: &ServerSpec) -> String {
    format!("http://{}:{}/v1", spec.host, spec.port)
}

/// Localhost predicate. Treats `localhost`, `127.0.0.1`, `::1` as
/// loopback. Everything else (incl. `0.0.0.0`) is rejected.
pub fn is_localhost(host: &str) -> bool {
    matches!(host, "localhost" | "127.0.0.1" | "::1")
}
