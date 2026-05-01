//! drive — driver trait + shared types + ManualDriver for `kei-spawn drive`.
//!
//! The `drive` subcommand is the one-call replacement for the current
//! two-step dance (`kei-spawn spawn` → orchestrator pastes Agent invocation).
//!
//! Two drivers live here:
//!   - `ManualDriver` — always returns `NotImplemented` (v0.1 default path).
//!   - `HttpDriver`   — real impl lives in `drive_http` behind feature
//!     `http-driver`; without the feature a stub returning
//!     `NotImplemented` preserves the v0.1 API surface.
//!
//! Exit-code contract (mirrors `kei-runtime::InvokeError::NotImplemented`):
//!   - 64 (EX_USAGE range) when the driver yields `NotImplemented`
//!   - 1 on spawn failure (same as `kei-spawn spawn`)
//!   - 0 only when a real driver returns Ok
//!
//! Constructor Pattern: one trait + two zero-state impls + one helper fn.

use serde::Serialize;

/// Success envelope for the `HttpDriver` (and the contract
/// `ManualDriver` deliberately never fulfils).
#[derive(Debug, Clone, Serialize)]
pub struct AgentResult {
    pub agent_id: String,
    pub transcript: String,
    pub finish_reason: String,
}

/// Errors surfaced from driver invocation.
#[derive(Debug)]
pub enum DriveError {
    NotImplemented { reason: String },
    Transport { message: String },
}

impl std::fmt::Display for DriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented { reason } => {
                write!(f, "kei-spawn drive: {reason}")
            }
            Self::Transport { message } => {
                write!(f, "kei-spawn drive transport: {message}")
            }
        }
    }
}

impl std::error::Error for DriveError {}

/// Abstraction over "how does an agent invocation actually happen."
pub trait AnthropicDriver {
    fn invoke(
        &self,
        prompt: &str,
        subagent_type: &str,
        isolation: Option<&str>,
    ) -> Result<AgentResult, DriveError>;
}

/// v0.1 driver — returns `NotImplemented` unconditionally.
pub struct ManualDriver;

impl AnthropicDriver for ManualDriver {
    fn invoke(
        &self,
        _prompt: &str,
        _subagent_type: &str,
        _isolation: Option<&str>,
    ) -> Result<AgentResult, DriveError> {
        Err(DriveError::NotImplemented {
            reason: not_implemented_message(),
        })
    }
}

/// Stub `HttpDriver` used when the `http-driver` feature is OFF.
///
/// Keeps the public API stable so downstream crates can name the type
/// unconditionally. Returns `NotImplemented` with a clear message pointing
/// to the feature flag.
#[cfg(not(feature = "http-driver"))]
pub struct HttpDriver;

#[cfg(not(feature = "http-driver"))]
impl AnthropicDriver for HttpDriver {
    fn invoke(
        &self,
        _prompt: &str,
        _subagent_type: &str,
        _isolation: Option<&str>,
    ) -> Result<AgentResult, DriveError> {
        Err(DriveError::NotImplemented {
            reason: "HttpDriver requires `--features http-driver`; \
                     rebuild with it to enable Anthropic-API calls"
                .to_string(),
        })
    }
}

/// Re-export real `HttpDriver` when feature is ON.
#[cfg(feature = "http-driver")]
pub use crate::drive_http::HttpDriver;

/// Canonical stderr message for the v0.1 stub.
pub fn not_implemented_message() -> String {
    "HTTP Anthropic-API integration not yet wired; use spawn then manual \
     Agent-tool invocation (see printed instructions)"
        .to_string()
}

/// Drive helper — orchestrator-facing entry that dispatches to a driver.
pub fn drive_with<D: AnthropicDriver>(
    driver: &D,
    prompt: &str,
    subagent_type: &str,
    isolation: Option<&str>,
) -> Result<AgentResult, DriveError> {
    driver.invoke(prompt, subagent_type, isolation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_driver_returns_not_implemented() {
        let d = ManualDriver;
        let err = d.invoke("p", "code-implementer", Some("worktree")).unwrap_err();
        match err {
            DriveError::NotImplemented { reason } => {
                assert!(reason.contains("HTTP"), "reason: {reason}");
            }
            other => panic!("expected NotImplemented, got {other}"),
        }
    }

    #[cfg(not(feature = "http-driver"))]
    #[test]
    fn http_driver_stub_returns_not_implemented_without_feature() {
        let d = HttpDriver;
        assert!(matches!(
            d.invoke("p", "x", None),
            Err(DriveError::NotImplemented { .. })
        ));
    }
}
